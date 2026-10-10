import React, { forwardRef, SelectHTMLAttributes, useState, useRef, useEffect } from 'react';
import { Icon, IconName } from './Icon';
import { cva, type VariantProps } from 'class-variance-authority';

// Select variants
const selectVariants = cva(
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

// Select size and icon size types
type SelectSize = 'xs' | 'sm' | 'm' | 'l';
type IconSize = 'xs' | 'sm' | 'm' | 'l' | 'xl';

// Option props
export interface SelectOption {
  value: string;
  label: string;
  icon?: IconName;
  disabled?: boolean;
}

// Select props - extend SelectHTMLAttributes without conflicting 'size' and 'onChange' properties
interface BaseSelectProps {
  options: SelectOption[];
  leftIcon?: IconName;
  rightIcon?: IconName;
  error?: string;
  label?: string;
  helpText?: string;
  onChange?: (value: string) => void;
  searchable?: boolean;
  placeholder?: string;
}

export interface SelectProps extends 
  Omit<SelectHTMLAttributes<HTMLSelectElement>, 'onChange' | 'size'>,
  BaseSelectProps,
  VariantProps<typeof selectVariants> {
}

// Select component
const Select = forwardRef<HTMLSelectElement, SelectProps>(
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
      options,
      onChange,
      searchable = false,
      placeholder,
      value,
      ...props
    },
    ref
  ) => {
    const [isOpen, setIsOpen] = useState(false);
    const [searchTerm, setSearchTerm] = useState('');
    const selectRef = useRef<HTMLSelectElement>(null);
    const wrapperRef = useRef<HTMLDivElement>(null);
    
    const hasErrorState = hasError || !!error;

    // Close dropdown when clicking outside
    useEffect(() => {
      const handleClickOutside = (event: MouseEvent) => {
        if (wrapperRef.current && !wrapperRef.current.contains(event.target as Node)) {
          setIsOpen(false);
        }
      };

      if (isOpen) {
        document.addEventListener('mousedown', handleClickOutside);
      }

      return () => {
        document.removeEventListener('mousedown', handleClickOutside);
      };
    }, [isOpen]);

    // Close dropdown on escape key
    useEffect(() => {
      const handleKeyDown = (event: KeyboardEvent) => {
        if (event.key === 'Escape') {
          setIsOpen(false);
        }
      };

      document.addEventListener('keydown', handleKeyDown);
      return () => document.removeEventListener('keydown', handleKeyDown);
    }, []);

    const handleSelect = (selectedValue: string) => {
      if (onChange && !disabled) {
        onChange(selectedValue);
      }
      setIsOpen(false);
    };

    // Map size to icon size safely
    const iconSize: IconSize = size === 'xs' ? 'xs' : size === 'l' ? 'l' : 'm';

    const filteredOptions = searchable 
      ? options.filter(option => 
          option.label.toLowerCase().includes(searchTerm.toLowerCase()) ||
          option.value.toLowerCase().includes(searchTerm.toLowerCase())
        )
      : options;

    const selectedLabel = options.find(opt => opt.value === value)?.label || placeholder || 'Sélectionner';

    if (searchable) {
      return (
        <div className="w-full" ref={wrapperRef}>
          {label && (
            <label className="block text-body-s font-medium text-primary-200 mb-1.5">
              {label}
            </label>
          )}
          
          <div className="relative">
            <div
              className={`relative ${selectVariants({ variant, size, hasError: hasErrorState, className })} cursor-pointer`}
              onClick={() => !disabled && setIsOpen(!isOpen)}
            >
              {leftIcon && (
                <Icon
                  name={leftIcon}
                  size={iconSize}
                  className="absolute left-3 top-1/2 -translate-y-1/2 text-primary-300 pointer-events-none"
                />
              )}
              
              <div className={`flex items-center justify-between ${leftIcon ? 'pl-10' : 'pl-4'} ${rightIcon ? 'pr-10' : 'pr-4'}`}>
                <span className="truncate">{selectedLabel}</span>
                <Icon 
                  name={isOpen ? 'ChevronUp' : 'ChevronDown'}
                  size={iconSize}
                  className="text-primary-300"
                />
              </div>
              
              {isOpen && (
                <div className="absolute top-full left-0 mt-1 w-full min-w-[200px] bg-primary-800 border border-primary-700 rounded-lg shadow-lg z-50">
                  {searchable && (
                    <div className="p-2 border-b border-primary-700">
                      <div className="relative">
                        <Icon 
                          name="Search" 
                          size="sm" 
                          className="absolute left-3 top-1/2 -translate-y-1/2 text-primary-300"
                        />
                        <input
                          type="text"
                          value={searchTerm}
                          onChange={(e) => setSearchTerm(e.target.value)}
                          placeholder="Rechercher..."
                          className="w-full pl-10 pr-4 py-2 bg-primary-900 border border-primary-700 rounded-md text-white placeholder:text-primary-400 focus:outline-none focus:ring-2 focus:ring-gold-700/50 text-sm"
                          autoFocus
                        />
                      </div>
                    </div>
                  )}
                  
                  <div className="max-h-60 overflow-y-auto">
                    {filteredOptions.length > 0 ? (
                      filteredOptions.map(option => (
                        <button
                          key={option.value}
                          type="button"
                          onClick={() => handleSelect(option.value)}
                          disabled={option.disabled || disabled}
                          className={`w-full px-4 py-2 text-left text-sm flex items-center gap-2 hover:bg-primary-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed ${value === option.value ? 'bg-primary-700' : ''}`}
                        >
                          {option.icon && <Icon name={option.icon} size="sm" />}
                          <span>{option.label}</span>
                        </button>
                      ))
                    ) : (
                      <div className="px-4 py-2 text-center text-primary-400 text-sm">
                        Aucune option trouvée
                      </div>
                    )}
                  </div>
                </div>
              )}
            </div>
            
            <select
              ref={selectRef}
              className="sr-only"
              value={value}
              onChange={(e) => onChange?.(e.target.value)}
              disabled={disabled}
              {...props}
            >
              {placeholder && <option value="" disabled hidden>{placeholder}</option>}
              {options.map(option => (
                <option key={option.value} value={option.value} disabled={option.disabled}>
                  {option.label}
                </option>
              ))}
            </select>
          </div>
          
          {error && (
            <p className="text-caption text-danger-400 mt-1 flex items-center gap-1">
              <Icon name="AlertCircle" size="xs" />
              {error}
            </p>
          )}
          
          {helpText && !error && (
            <p id={`${props.id}-help`} className="text-caption text-primary-400 mt-1">
              {helpText}
            </p>
          )}
        </div>
      );
    }

    // Standard select
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
          
          <select
            ref={ref}
            className={selectVariants({
              variant,
              size: size as SelectSize,
              hasError: hasErrorState,
              className: `${leftIcon ? 'pl-10' : 'pl-4'} ${rightIcon ? 'pr-10' : 'pr-4'} ${className || ''}`,
            })}
            disabled={disabled}
            onChange={(e) => onChange?.(e.target.value)}
            value={value}
            aria-invalid={hasErrorState}
            aria-describedby={helpText ? `${props.id}-help` : undefined}
            {...props}
          >
            {placeholder && <option value="" disabled hidden>{placeholder}</option>}
            {options.map(option => (
              <option key={option.value} value={option.value} disabled={option.disabled}>
                {option.label}
              </option>
            ))}
          </select>
          
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
          <p id={`${props.id}-help`} className="text-caption text-primary-400 mt-1">
            {helpText}
          </p>
        )}
      </div>
    );
  }
);

Select.displayName = 'Select';

export default Select;
export { selectVariants };
