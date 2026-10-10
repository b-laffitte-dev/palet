import React, { forwardRef, ReactNode } from 'react';
import { Icon, IconName } from './Icon';
import Button, { ButtonProps } from './Button';
import Input from './Input';
import Select, { SelectOption } from './Select';
import { cva } from 'class-variance-authority';

// Import types for reference
type InputProps = any;
type SelectProps = any;

// Form container variants
const formContainerVariants = cva(
  'w-full space-y-6',
  {
    variants: {
      layout: {
        default: 'space-y-6',
        compact: 'space-y-4',
        tight: 'space-y-2',
      },
    },
    defaultVariants: {
      layout: 'default',
    },
  }
);

// Form field variants
const formFieldVariants = cva(
  'space-y-2',
  {
    variants: {
      disabled: {
        true: 'opacity-50',
        false: '',
      },
    },
    defaultVariants: {
      disabled: false,
    },
  }
);

// Form types
export type FormLayout = 'default' | 'compact' | 'tight';
export type FormSize = 'xs' | 'sm' | 'm' | 'l';
export type FormState = 'idle' | 'loading' | 'success' | 'error';

// Field types
export interface FormFieldBase {
  id: string;
  label?: string;
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  readOnly?: boolean;
  helpText?: string;
  error?: string;
  className?: string;
  labelClassName?: string;
  errorClassName?: string;
  helpTextClassName?: string;
}

export interface TextFieldProps extends FormFieldBase {
  type?: 'text' | 'email' | 'password' | 'number' | 'tel' | 'url' | 'search';
  value?: string;
  onChange?: (value: string) => void;
  leftIcon?: IconName;
  rightIcon?: IconName;
  min?: number;
  max?: number;
  pattern?: string;
  autoComplete?: string;
}

export interface SelectFieldProps extends FormFieldBase {
  options: SelectOption[];
  value?: string;
  onChange?: (value: string) => void;
  multiple?: boolean;
  searchable?: boolean;
  leftIcon?: IconName;
  rightIcon?: IconName;
}

export interface TextAreaFieldProps extends FormFieldBase {
  value?: string;
  onChange?: (value: string) => void;
  rows?: number;
  cols?: number;
  maxLength?: number;
  autoResize?: boolean;
}

export interface CheckboxFieldProps extends FormFieldBase {
  value?: boolean;
  onChange?: (value: boolean) => void;
  labelPlacement?: 'left' | 'right';
  size?: FormSize;
}

export interface RadioFieldProps extends FormFieldBase {
  options: { value: string; label: string; icon?: IconName; disabled?: boolean }[];
  value?: string;
  onChange?: (value: string) => void;
  direction?: 'horizontal' | 'vertical';
  size?: FormSize;
}

export interface SwitchFieldProps extends FormFieldBase {
  value?: boolean;
  onChange?: (value: boolean) => void;
  size?: FormSize;
  switchLabel?: string;
}

export interface DateFieldProps extends FormFieldBase {
  value?: string | Date;
  onChange?: (value: string | Date) => void;
  min?: string | Date;
  max?: string | Date;
  showTime?: boolean;
  size?: FormSize;
}

// Field components
const TextField = forwardRef<HTMLInputElement, TextFieldProps>(
  (
    {
      id,
      label,
      placeholder,
      required,
      disabled,
      readOnly,
      helpText,
      error,
      className,
      labelClassName,
      errorClassName,
      helpTextClassName,
      type = 'text',
      value,
      onChange,
      leftIcon,
      rightIcon,
      ...props
    },
    ref
  ) => {
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      onChange?.(e.target.value);
    };

    return (
      <div className={formFieldVariants({ disabled })}>
        {label && (
          <label htmlFor={id} className={`block text-body-s font-medium text-primary-200 ${required ? 'after:content-["*"] after:text-danger-500 after:ml-1' : ''} ${labelClassName || ''}`}>
            {label}
          </label>
        )}
        
        <Input
          ref={ref}
          id={id}
          type={type}
          value={value}
          onChange={handleChange}
          placeholder={placeholder}
          disabled={disabled}
          readOnly={readOnly}
          required={required}
          hasError={!!error}
          error={error}
          leftIcon={leftIcon}
          rightIcon={rightIcon}
          className={className}
          aria-describedby={helpText ? `${id}-help` : error ? `${id}-error` : undefined}
          {...props}
        />
        
        {error && (
          <p id={`${id}-error`} className={`text-caption text-danger-400 mt-1 flex items-center gap-1 ${errorClassName || ''}`}>
            <Icon name="AlertCircle" size="xs" />
            {error}
          </p>
        )}
        
        {helpText && !error && (
          <p id={`${id}-help`} className={`text-caption text-primary-400 mt-1 ${helpTextClassName || ''}`}>
            {helpText}
          </p>
        )}
      </div>
    );
  }
);

TextField.displayName = 'TextField';

const SelectField = forwardRef<HTMLSelectElement, SelectFieldProps>(
  (
    {
      id,
      label,
      placeholder,
      required,
      disabled,
      readOnly,
      helpText,
      error,
      className,
      labelClassName,
      errorClassName,
      helpTextClassName,
      options,
      value,
      onChange,
      leftIcon,
      rightIcon,
      searchable,
      ...props
    },
    ref
  ) => {
    const handleChange = (selectedValue: string) => {
      onChange?.(selectedValue);
    };

    return (
      <div className={formFieldVariants({ disabled })}>
        {label && (
          <label htmlFor={id} className={`block text-body-s font-medium text-primary-200 ${required ? 'after:content-["*"] after:text-danger-500 after:ml-1' : ''} ${labelClassName || ''}`}>
            {label}
          </label>
        )}
        
        <Select
          ref={ref}
          id={id}
          options={options}
          value={value}
          onChange={handleChange}
          disabled={disabled}
          hasError={!!error}
          error={error}
          placeholder={placeholder}
          leftIcon={leftIcon}
          rightIcon={rightIcon}
          searchable={searchable}
          className={className}
          aria-describedby={helpText ? `${id}-help` : error ? `${id}-error` : undefined}
          {...props}
        />
        
        {error && (
          <p id={`${id}-error`} className={`text-caption text-danger-400 mt-1 flex items-center gap-1 ${errorClassName || ''}`}>
            <Icon name="AlertCircle" size="xs" />
            {error}
          </p>
        )}
        
        {helpText && !error && (
          <p id={`${id}-help`} className={`text-caption text-primary-400 mt-1 ${helpTextClassName || ''}`}>
            {helpText}
          </p>
        )}
      </div>
    );
  }
);

SelectField.displayName = 'SelectField';

const TextAreaField = forwardRef<HTMLTextAreaElement, TextAreaFieldProps>(
  (
    {
      id,
      label,
      placeholder,
      required,
      disabled,
      readOnly,
      helpText,
      error,
      className,
      labelClassName,
      errorClassName,
      helpTextClassName,
      value,
      onChange,
      rows = 4,
      maxLength,
      ...props
    },
    ref
  ) => {
    const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
      onChange?.(e.target.value);
    };

    const characterCount = value?.length || 0;
    const showCounter = maxLength && characterCount > 0;

    return (
      <div className={formFieldVariants({ disabled })}>
        {label && (
          <label htmlFor={id} className={`block text-body-s font-medium text-primary-200 ${required ? 'after:content-["*"] after:text-danger-500 after:ml-1' : ''} ${labelClassName || ''}`}>
            {label}
          </label>
        )}
        
        <textarea
          ref={ref}
          id={id}
          value={value}
          onChange={handleChange}
          placeholder={placeholder}
          disabled={disabled}
          readOnly={readOnly}
          required={required}
          rows={rows}
          maxLength={maxLength}
          className={`w-full px-4 py-2 bg-primary-900 border ${!!error ? 'border-danger-700' : 'border-primary-700'} rounded-lg text-white placeholder:text-primary-400 focus:outline-none focus:ring-2 focus:ring-gold-700/50 focus:border-gold-700 transition-all disabled:opacity-50 disabled:cursor-not-allowed resize-vertical ${className || ''}`}
          aria-describedby={helpText ? `${id}-help` : error ? `${id}-error` : undefined}
          {...props}
        />
        
        {showCounter && (
          <div className="flex justify-between">
            {error && (
              <p id={`${id}-error`} className={`text-caption text-danger-400 mt-1 flex items-center gap-1 ${errorClassName || ''}`}>
                <Icon name="AlertCircle" size="xs" />
                {error}
              </p>
            )}
            <p className={`text-caption text-primary-400 mt-1 ${maxLength && characterCount > maxLength ? 'text-danger-400' : ''}`}>
              {characterCount}/{maxLength}
            </p>
          </div>
        )}
        
        {!showCounter && error && (
          <p id={`${id}-error`} className={`text-caption text-danger-400 mt-1 flex items-center gap-1 ${errorClassName || ''}`}>
            <Icon name="AlertCircle" size="xs" />
            {error}
          </p>
        )}
        
        {helpText && !error && !showCounter && (
          <p id={`${id}-help`} className={`text-caption text-primary-400 mt-1 ${helpTextClassName || ''}`}>
            {helpText}
          </p>
        )}
      </div>
    );
  }
);

TextAreaField.displayName = 'TextAreaField';

const CheckboxField = forwardRef<HTMLInputElement, CheckboxFieldProps>(
  (
    {
      id,
      label,
      required,
      disabled,
      readOnly,
      helpText,
      error,
      className,
      labelClassName,
      errorClassName,
      helpTextClassName,
      value = false,
      onChange,
      labelPlacement = 'right',
      size = 'm',
      ...props
    },
    ref
  ) => {
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      onChange?.(e.target.checked);
    };

    const getSizeClasses = () => {
      switch (size) {
        case 'xs': return 'w-3 h-3';
        case 'sm': return 'w-4 h-4';
        case 'l': return 'w-6 h-6';
                default: return 'w-4 h-4';
      }
    };

    return (
      <div className={formFieldVariants({ disabled })}>
        <div className={`flex items-center gap-2 ${labelPlacement === 'left' ? 'flex-row-reverse' : ''}`}>
          <input
            ref={ref}
            id={id}
            type="checkbox"
            checked={value}
            onChange={handleChange}
            disabled={disabled || readOnly}
            required={required}
            className={`rounded border-primary-600 bg-transparent focus:ring-2 focus:ring-gold-500 focus:border-gold-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed ${getSizeClasses()} ${className || ''}`}
            aria-describedby={helpText ? `${id}-help` : error ? `${id}-error` : undefined}
            {...props}
          />
          
          {label && (
            <label htmlFor={id} className={`text-body-s text-primary-200 cursor-pointer ${disabled ? 'opacity-50 cursor-not-allowed' : ''} ${labelClassName || ''}`}>
              {label}
              {required && <span className="text-danger-500 ml-1">*</span>}
            </label>
          )}
        </div>
        
        {error && (
          <p id={`${id}-error`} className={`text-caption text-danger-400 mt-1 flex items-center gap-1 ${errorClassName || ''}`}>
            <Icon name="AlertCircle" size="xs" />
            {error}
          </p>
        )}
        
        {helpText && !error && (
          <p id={`${id}-help`} className={`text-caption text-primary-400 mt-1 ${helpTextClassName || ''}`}>
            {helpText}
          </p>
        )}
      </div>
    );
  }
);

CheckboxField.displayName = 'CheckboxField';

const RadioField = forwardRef<HTMLInputElement, RadioFieldProps>(
  (
    {
      id,
      label,
      required,
      disabled,
      readOnly,
      helpText,
      error,
      className,
      labelClassName,
      errorClassName,
      helpTextClassName,
      options,
      value,
      onChange,
      direction = 'vertical',
      size = 'm',
      ...props
    },
    ref
  ) => {
    const handleChange = (selectedValue: string) => {
      onChange?.(selectedValue);
    };

    const getSizeClasses = () => {
      switch (size) {
        case 'xs': return 'w-3 h-3';
        case 'sm': return 'w-4 h-4';
        case 'l': return 'w-6 h-6';
                default: return 'w-4 h-4';
      }
    };

    const directionClasses = {
      horizontal: 'flex-row gap-4',
      vertical: 'flex-col gap-2',
    };

    return (
      <div className={formFieldVariants({ disabled })}>
        {label && (
          <label className={`block text-body-s font-medium text-primary-200 mb-2 ${required ? 'after:content-["*"] after:text-danger-500 after:ml-1' : ''} ${labelClassName || ''}`}>
            {label}
          </label>
        )}
        
        <div className={`flex ${directionClasses[direction]}`}>
          {options.map((option) => (
            <div key={option.value} className="flex items-center gap-2">
              <input
                ref={ref}
                type="radio"
                id={`${id}-${option.value}`}
                name={id}
                value={option.value}
                checked={value === option.value}
                onChange={() => handleChange(option.value)}
                disabled={disabled || readOnly || option.disabled}
                required={required}
                className={`rounded-full border-primary-600 bg-transparent focus:ring-2 focus:ring-gold-500 focus:border-gold-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed ${getSizeClasses()} ${className || ''}`}
                {...props}
              />
              
              <label 
                htmlFor={`${id}-${option.value}`}
                className={`text-body-s text-primary-200 cursor-pointer ${(disabled || readOnly || option.disabled) ? 'opacity-50 cursor-not-allowed' : ''}`}
              >
                {option.icon && <Icon name={option.icon} size={size === 'xs' ? 'xs' : size === 'l' ? 'l' : 'm'} className="mr-1" />}
                {option.label}
              </label>
            </div>
          ))}
        </div>
        
        {error && (
          <p id={`${id}-error`} className={`text-caption text-danger-400 mt-1 flex items-center gap-1 ${errorClassName || ''}`}>
            <Icon name="AlertCircle" size="xs" />
            {error}
          </p>
        )}
        
        {helpText && !error && (
          <p id={`${id}-help`} className={`text-caption text-primary-400 mt-1 ${helpTextClassName || ''}`}>
            {helpText}
          </p>
        )}
      </div>
    );
  }
);

RadioField.displayName = 'RadioField';

const SwitchField = forwardRef<HTMLInputElement, SwitchFieldProps>(
  (
    {
      id,
      label,
      required,
      disabled,
      readOnly,
      helpText,
      error,
      className,
      labelClassName,
      errorClassName,
      helpTextClassName,
      value = false,
      onChange,
      size = 'm',
      switchLabel,
      ...props
    },
    ref
  ) => {
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      onChange?.(e.target.checked);
    };

    const getSizeClasses = () => {
      switch (size) {
        case 'xs': return 'w-8 h-4';
        case 'sm': return 'w-10 h-5';
        case 'l': return 'w-12 h-6';
                default: return 'w-10 h-5';
      }
    };

    const getThumbSize = () => {
      switch (size) {
        case 'xs': return 'w-3 h-3';
        case 'sm': return 'w-4 h-4';
        case 'l': return 'w-5 h-5';
                default: return 'w-4 h-4';
      }
    };

    return (
      <div className={formFieldVariants({ disabled })}>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            {label && (
              <label htmlFor={id} className={`text-body-s font-medium text-primary-200 ${required ? 'after:content-["*"] after:text-danger-500 after:ml-1' : ''} ${labelClassName || ''}`}>
                {label}
              </label>
            )}
            
            {switchLabel && (
              <label htmlFor={id} className="text-body-s text-primary-300">{switchLabel}</label>
            )}
          </div>
          
          <button
            type="button"
            role="switch"
            aria-checked={value}
            onClick={() => !disabled && !readOnly && onChange?.(!value)}
            disabled={disabled || readOnly}
            className={`relative inline-flex items-center ${getSizeClasses()} rounded-full transition-all focus:outline-none focus:ring-2 focus:ring-gold-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed ${value ? 'bg-gold-500' : 'bg-primary-600'} ${className || ''}`}
          >
            <span 
              className={`absolute ${getThumbSize()} bg-white rounded-full shadow-md transition-transform ${value ? 'translate-x-full' : 'translate-x-0'}`}
            />
          </button>
        </div>
        
        {error && (
          <p id={`${id}-error`} className={`text-caption text-danger-400 mt-1 flex items-center gap-1 ${errorClassName || ''}`}>
            <Icon name="AlertCircle" size="xs" />
            {error}
          </p>
        )}
        
        {helpText && !error && (
          <p id={`${id}-help`} className={`text-caption text-primary-400 mt-1 ${helpTextClassName || ''}`}>
            {helpText}
          </p>
        )}
      </div>
    );
  }
);

SwitchField.displayName = 'SwitchField';

const DateField = forwardRef<HTMLInputElement, DateFieldProps>(
  (
    {
      id,
      label,
      placeholder,
      required,
      disabled,
      readOnly,
      helpText,
      error,
      className,
      labelClassName,
      errorClassName,
      helpTextClassName,
      value,
      onChange,
      min,
      max,
      showTime = false,
      size = 'm',
      ...props
    },
    ref
  ) => {
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      onChange?.(e.target.value);
    };

    const formatDate = (date: string | Date | undefined): string => {
      if (!date) return '';
      const d = typeof date === 'string' ? new Date(date) : date;
      if (showTime) {
        return d.toISOString().slice(0, 16); // YYYY-MM-DDTHH:MM
      }
      return d.toISOString().slice(0, 10); // YYYY-MM-DD
    };

    return (
      <div className={formFieldVariants({ disabled })}>
        {label && (
          <label htmlFor={id} className={`block text-body-s font-medium text-primary-200 ${required ? 'after:content-["*"] after:text-danger-500 after:ml-1' : ''} ${labelClassName || ''}`}>
            {label}
          </label>
        )}
        
        <input
          ref={ref}
          id={id}
          type={showTime ? 'datetime-local' : 'date'}
          value={formatDate(value)}
          onChange={handleChange}
          placeholder={placeholder}
          disabled={disabled}
          readOnly={readOnly}
          required={required}
          min={min ? formatDate(min) : undefined}
          max={max ? formatDate(max) : undefined}
          className={`w-full px-4 py-2 bg-primary-900 border ${!!error ? 'border-danger-700' : 'border-primary-700'} rounded-lg text-white placeholder:text-primary-400 focus:outline-none focus:ring-2 focus:ring-gold-700/50 focus:border-gold-700 transition-all disabled:opacity-50 disabled:cursor-not-allowed ${className || ''}`}
          aria-describedby={helpText ? `${id}-help` : error ? `${id}-error` : undefined}
          {...props}
        />
        
        {error && (
          <p id={`${id}-error`} className={`text-caption text-danger-400 mt-1 flex items-center gap-1 ${errorClassName || ''}`}>
            <Icon name="AlertCircle" size="xs" />
            {error}
          </p>
        )}
        
        {helpText && !error && (
          <p id={`${id}-help`} className={`text-caption text-primary-400 mt-1 ${helpTextClassName || ''}`}>
            {helpText}
          </p>
        )}
      </div>
    );
  }
);

DateField.displayName = 'DateField';

// Form component with form-level state
export interface FormProps extends React.FormHTMLAttributes<HTMLFormElement> {
  children: React.ReactNode;
  layout?: FormLayout;
  state?: FormState;
  onSubmit?: (e: React.FormEvent<HTMLFormElement>) => void;
  submitButton?: React.ReactNode;
  submitText?: string;
  submitDisabled?: boolean;
  submitLoading?: boolean;
  submitIcon?: IconName;
  submitVariant?: ButtonProps['variant'];
  submitSize?: ButtonProps['size'];
  className?: string;
  onCancel?: () => void;
  cancelText?: string;
  cancelVariant?: ButtonProps['variant'];
  showCancel?: boolean;
}

const Form = forwardRef<HTMLFormElement, FormProps>(
  (
    {
      children,
      layout = 'default',
      state = 'idle',
      onSubmit,
      submitButton,
      submitText = 'Soumettre',
      submitDisabled = false,
      submitLoading = false,
      submitIcon = 'Check',
      submitVariant = 'primary',
      submitSize = 'm',
      className,
      onCancel,
      cancelText = 'Annuler',
      cancelVariant = 'outline',
      showCancel = false,
      ...props
    },
    ref
  ) => {
    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
      if (onSubmit && !submitDisabled && !submitLoading) {
        onSubmit(e);
      }
    };

    return (
      <form 
        ref={ref}
        onSubmit={handleSubmit}
        className={formContainerVariants({ layout, className })}
        {...props}
      >
        {children}
        
        {/* Form footer with buttons */}
        {(submitButton || submitText || showCancel) && (
          <div className="flex items-center justify-end gap-3 pt-4 border-t border-primary-700/50">
            {showCancel && onCancel && (
              <Button 
                type="button"
                variant={cancelVariant}
                size={submitSize}
                onClick={onCancel}
                disabled={state === 'loading' || submitLoading}
              >
                {cancelText}
              </Button>
            )}
            
            {submitButton || (
              <Button 
                type="submit"
                variant={submitVariant}
                size={submitSize}
                disabled={submitDisabled || state === 'loading' || submitLoading}
                loading={submitLoading || state === 'loading'}
                leftIcon={submitIcon}
              >
                {submitText}
              </Button>
            )}
          </div>
        )}
      </form>
    );
  }
);

Form.displayName = 'Form';

// Formimperative handle for form reset and validation
export interface FormImperativeHandle {
  reset: () => void;
  validate: () => boolean;
  getValues: () => Record<string, any>;
  setValues: (values: Record<string, any>) => void;
  setFieldError: (field: string, error: string) => void;
  clearFieldError: (field: string) => void;
  submit: () => void;
}

// Field group for grouping related fields
export interface FieldGroupProps {
  children: ReactNode;
  label?: string;
  description?: string;
  className?: string;
  layout?: 'default' | 'compact' | 'tight';
}

const FieldGroup = forwardRef<HTMLDivElement, FieldGroupProps>(
  (
    {
      children,
      label,
      description,
      className,
      layout = 'default',
      ...props
    },
    ref
  ) => {
    return (
      <div ref={ref} className={`space-y-4 ${className || ''}`} {...props}>
        {label && (
          <div className="flex flex-col gap-1">
            <h3 className="text-h6 font-semibold text-white">{label}</h3>
            {description && (
              <p className="text-body-s text-primary-300">{description}</p>
            )}
          </div>
        )}
        <div className={formContainerVariants({ layout })}>
          {children}
        </div>
      </div>
    );
  }
);

FieldGroup.displayName = 'FieldGroup';

// Form validation error summary
export interface FormErrorSummaryProps {
  errors: string[];
  title?: string;
  className?: string;
}

const FormErrorSummary: React.FC<FormErrorSummaryProps> = ({
  errors,
  title = 'Corrigez les erreurs suivantes',
  className,
}) => {
  if (errors.length === 0) return null;

  return (
    <div className={`bg-danger-500/10 border border-danger-500/20 rounded-lg p-4 ${className || ''}`}>
      <h4 className="text-h6 font-semibold text-danger-400 mb-2">{title}</h4>
      <ul className="list-disc list-inside space-y-1">
        {errors.map((error, index) => (
          <li key={index} className="text-body-s text-danger-400">
            {error}
          </li>
        ))}
      </ul>
    </div>
  );
};

// Exports
export {
  TextField,
  SelectField,
  TextAreaField,
  CheckboxField,
  RadioField,
  SwitchField,
  DateField,
  FieldGroup,
  FormErrorSummary,
};

export default Form;
