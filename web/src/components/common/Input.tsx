import React, { forwardRef, InputHTMLAttributes } from 'react';
import { Icon, IconName } from './Icon';
import { cva, type VariantProps } from 'class-variance-authority';

// Input variants using CVA
const inputVariants = cva(
  'inline-flex items-center w-full rounded-lg transition-all focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed',
  {
    variants: {
      variant: {
        default: 'bg-primary-900 border border-primary-700 text-white placeholder:text-primary-400 focus:border-gold-700 focus:ring-gold-700/50',
        ghost: 'bg-transparent border-0 text-white placeholder:text-primary-400 focus:ring-0',
        outline: 'bg-transparent border border-primary-700 text-white placeholder:text-primary-400 focus:border-gold-700 focus:ring-gold-700/50',
      },
      size: {
        xs: 'px-2 py-1 text-xs',
        sm: 'px-3 py-1.5 text-sm',
        m: 'px-4 py-2 text-sm',
        l: 'px-6 py-3 text-base',
      },
      hasError: {
        true: 'border-danger-700 focus:border-danger-600 focus:ring-danger-700/50',
        false: '',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'm',
      hasError: false,
    },
  }
);

// Explicit size type for Input
type InputSize = 'xs' | 'sm' | 'm' | 'l';
type IconSize = 'xs' | 'sm' | 'm' | 'l' | 'xl';


// Input props - extend InputHTMLAttributes without conflicting 'size' property
interface BaseInputProps {
  leftIcon?: IconName;
  rightIcon?: IconName;
  error?: string;
  label?: string;
  helpText?: string;
}

export interface InputProps extends 
  Omit<InputHTMLAttributes<HTMLInputElement>, 'size'>,
  BaseInputProps,
  VariantProps<typeof inputVariants> {
}

// Input component
const Input = forwardRef<HTMLInputElement, InputProps>(
  (
    {
      className,
      variant = 'default',
      size = 'm',
      hasError = false,
      leftIcon,
      rightIcon,
      error,
      label,
      helpText,
      disabled,
      ...props
    },
    ref
  ) => {
    const hasErrorState = hasError || !!error;
    
    // Map size to icon size safely
    const iconSize: IconSize = size === 'xs' ? 'xs' : size === 'l' ? 'l' : 'm';

    return (
      <div className="w-full">
        {label && (
          <label className="block text-body-s font-medium text-primary-200 mb-1.5">
            {label}
          </label>
        )}
        
        <div className="relative">
          {leftIcon && (
            <Icon
              name={leftIcon}
              size={iconSize}
              className="absolute left-3 top-1/2 -translate-y-1/2 text-primary-300 pointer-events-none"
            />
          )}
          
          <input
            ref={ref}
            className={inputVariants({
              variant,
              size: size as InputSize,
              hasError: hasErrorState,
              className: `${leftIcon ? 'pl-10' : 'pl-4'} ${rightIcon ? 'pr-10' : 'pr-4'} ${className || ''}`,
            })}
            disabled={disabled}
            aria-invalid={hasErrorState}
            aria-describedby={helpText ? `${props.id}-help` : undefined}
            {...props}
          />
          
          {rightIcon && (
            <Icon
              name={rightIcon}
              size={iconSize}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-primary-300 pointer-events-none"
            />
          )}
        </div>
        
        {error && (
          <p className="text-caption text-danger-400 mt-1 flex items-center gap-1">
            <Icon name="AlertCircle" size="xs" />
            {error}
          </p>
        )}
        
        {helpText && !error && (
          <p 
            id={`${props.id}-help`} 
            className="text-caption text-primary-400 mt-1"
          >
            {helpText}
          </p>
        )}
      </div>
    );
  }
);

Input.displayName = 'Input';

export default Input;

export { inputVariants };
