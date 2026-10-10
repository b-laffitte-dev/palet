import { forwardRef, ButtonHTMLAttributes, ReactNode } from 'react';
import { Icon, IconName } from './Icon';
import { cva, type VariantProps } from 'class-variance-authority';


// Button variants using CVA (Class Variance Authority pattern)
const buttonVariants = cva(
  'inline-flex items-center justify-center font-medium transition-all focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed',
  {
    variants: {
      variant: {
        primary: 'bg-primary-600 text-white hover:bg-primary-700 active:bg-primary-800',
        secondary: 'bg-neutral-100 text-neutral-900 hover:bg-neutral-200 active:bg-neutral-300',
        outline: 'border border-neutral-300 bg-transparent text-neutral-700 hover:bg-neutral-50 active:bg-neutral-100',
        ghost: 'bg-transparent text-neutral-600 hover:bg-neutral-100 active:bg-neutral-200',
        gold: 'bg-gold-600 text-white hover:bg-gold-700 active:bg-gold-800',
        danger: 'bg-danger-600 text-white hover:bg-danger-700 active:bg-danger-800',
        link: 'bg-transparent text-primary-600 hover:text-primary-700 hover:underline p-0',
      },
      size: {
        xs: 'px-2 py-1 text-xs gap-1',
        sm: 'px-3 py-1.5 text-sm gap-1.5',
        m: 'px-4 py-2 text-sm gap-2',
        l: 'px-6 py-3 text-base gap-2.5',
        xl: 'px-8 py-4 text-lg gap-3',
      },
      shape: {
        square: 'aspect-square',
        round: 'rounded-full',
        default: 'rounded-md',
      },
      fullWidth: {
        true: 'w-full',
        false: '',
      },
    },
    compoundVariants: [
      // Primary button with gold accent
      {
        variant: 'primary',
        className: 'shadow-sm hover:shadow-md active:shadow-none',
      },
      // Gold button with special styling
      {
        variant: 'gold',
        className: 'shadow-gold-sm hover:shadow-gold-m active:shadow-gold-l',
      },
      // Ghost button on dark background
      {
        variant: 'ghost',
        className: 'hover:text-neutral-900',
      },
    ],
    defaultVariants: {
      variant: 'primary',
      size: 'm',
      shape: 'default',
      fullWidth: false,
    },
  }
);

// Button props
export interface ButtonProps extends 
  ButtonHTMLAttributes<HTMLButtonElement>,
  VariantProps<typeof buttonVariants> {
  children: ReactNode;
  leftIcon?: IconName;
  rightIcon?: IconName;
  loading?: boolean;
  iconOnly?: boolean;
}

// Button component
const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      className,
      variant,
      size,
      shape,
      fullWidth,
      leftIcon,
      rightIcon,
      loading,
      iconOnly,
      disabled,
      children,
      ...props
    },
    ref
  ) => {
    // Combine custom styles with variant classes
    const baseStyles = buttonVariants({ variant, size, shape, fullWidth });
    
    return (
      <button
        ref={ref}
        className={`${baseStyles} ${className || ''}`}
        disabled={disabled || loading}
        {...props}
        style={{
          ...styles.base,
          ...(iconOnly && size && styles.iconOnly[size]),
          ...props.style,
        }}
      >
        {loading ? (
          <span className="animate-spin" style={styles.spinner}>
            <Icon name="Loader2" size="sm" />
          </span>
        ) : (
          <>
            {leftIcon && <Icon name={leftIcon} size={size || 'm'} />}
            {children && !iconOnly && <span>{children}</span>}
            {rightIcon && <Icon name={rightIcon} size={size || 'm'} />}
          </>
        )}
      </button>
    );
  }
);

Button.displayName = 'Button';

// Styles object
const styles = {
  base: {
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    transitionProperty: 'all',
    transitionDuration: '200ms',
    transitionTimingFunction: 'ease',
  } as React.CSSProperties,
  
  spinner: {
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
  } as React.CSSProperties,
  
  iconOnly: {
    xs: { padding: '0.25rem' },
    sm: { padding: '0.375rem' },
    m: { padding: '0.5rem' },
    l: { padding: '0.75rem' },
    xl: { padding: '1rem' },
  } as Record<string, React.CSSProperties>,
};

// Named exports for different button variants
export const ButtonPrimary = forwardRef<HTMLButtonElement, Omit<ButtonProps, 'variant'>>(
  (props, ref) => <Button ref={ref} variant="primary" {...props} />
);

ButtonPrimary.displayName = 'ButtonPrimary';

export const ButtonSecondary = forwardRef<HTMLButtonElement, Omit<ButtonProps, 'variant'>>(
  (props, ref) => <Button ref={ref} variant="secondary" {...props} />
);

ButtonSecondary.displayName = 'ButtonSecondary';

export const ButtonOutline = forwardRef<HTMLButtonElement, Omit<ButtonProps, 'variant'>>(
  (props, ref) => <Button ref={ref} variant="outline" {...props} />
);

ButtonOutline.displayName = 'ButtonOutline';

export const ButtonGhost = forwardRef<HTMLButtonElement, Omit<ButtonProps, 'variant'>>(
  (props, ref) => <Button ref={ref} variant="ghost" {...props} />
);

ButtonGhost.displayName = 'ButtonGhost';

export const ButtonGold = forwardRef<HTMLButtonElement, Omit<ButtonProps, 'variant'>>(
  (props, ref) => <Button ref={ref} variant="gold" {...props} />
);

ButtonGold.displayName = 'ButtonGold';

export const ButtonDanger = forwardRef<HTMLButtonElement, Omit<ButtonProps, 'variant'>>(
  (props, ref) => <Button ref={ref} variant="danger" {...props} />
);

ButtonDanger.displayName = 'ButtonDanger';

export const ButtonLink = forwardRef<HTMLButtonElement, Omit<ButtonProps, 'variant'>>(
  (props, ref) => <Button ref={ref} variant="link" {...props} />
);

ButtonLink.displayName = 'ButtonLink';

export default Button;