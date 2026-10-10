import { forwardRef, HTMLAttributes, ReactNode } from 'react';
import { cva, type VariantProps } from 'class-variance-authority';

// Card variants
const cardVariants = cva(
  'border border-neutral-200 bg-white rounded-lg overflow-hidden',
  {
    variants: {
      variant: {
        default: 'shadow-sm',
        flat: 'shadow-none',
        hoverable: 'hover:shadow-md transition-shadow',
        interactive: 'hover:shadow-md hover:-translate-y-1 transition-all cursor-pointer active:translate-y-0 active:shadow-sm',
        primary: 'bg-primary-600 text-white border-primary-600',
        secondary: 'bg-secondary-600 text-white border-secondary-600',
        subtle: 'bg-neutral-50 border-neutral-200',
      },
      padding: {
        none: '',
        sm: 'p-3',
        m: 'p-4',
        l: 'p-6',
        xl: 'p-8',
      },
      rounded: {
        none: 'rounded-none',
        sm: 'rounded-sm',
        m: 'rounded-md',
        l: 'rounded-lg',
        xl: 'rounded-xl',
        full: 'rounded-full',
      },
    },
    defaultVariants: {
      variant: 'default',
      padding: 'm',
      rounded: 'm',
    },
  }
);

// Card props
export interface CardProps extends 
  HTMLAttributes<HTMLDivElement>,
  VariantProps<typeof cardVariants> {
  children?: ReactNode;
  header?: ReactNode;
  footer?: ReactNode;
  image?: string;
  imageAlt?: string;
  title?: string;
  subtitle?: string;
}

// Card component
const Card = forwardRef<HTMLDivElement, CardProps>(
  (
    {
      className,
      variant,
      padding,
      rounded,
      children,
      header,
      footer,
      image,
      imageAlt = '',
      title,
      subtitle,
      ...props
    },
    ref
  ) => {
    const baseStyles = cardVariants({ variant, padding, rounded });

    return (
      <div
        ref={ref}
        className={`${baseStyles} ${className || ''}`}
        {...props}
      >
        {image && (
          <div style={styles.imageContainer}>
            <img 
              src={image} 
              alt={imageAlt}
              style={styles.image}
              loading="lazy"
            />
          </div>
        )}
        
        {(header || title || subtitle) && (
          <div style={styles.header}>
            {header}
            {!header && (
              <>
                {title && <h3 style={styles.title}>{title}</h3>}
                {subtitle && <p style={styles.subtitle}>{subtitle}</p>}
              </>
            )}
          </div>
        )}
        
        <div style={styles.content}>
          {children}
        </div>
        
        {footer && <div style={styles.footer}>{footer}</div>}
      </div>
    );
  }
);

Card.displayName = 'Card';

// Card Header component
const CardHeader = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={`px-4 pt-4 pb-0 ${className || ''}`} {...props} />
  )
);

CardHeader.displayName = 'CardHeader';

// Card Content component
const CardContent = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={`px-4 py-4 ${className || ''}`} {...props} />
  )
);

CardContent.displayName = 'CardContent';

// Card Footer component
const CardFooter = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={`px-4 py-3 bg-neutral-50 border-t border-neutral-200 ${className || ''}`} {...props} />
  )
);

CardFooter.displayName = 'CardFooter';

// Styles
const styles = {
  imageContainer: {
    position: 'relative' as const,
    overflow: 'hidden',
  } as React.CSSProperties,
  
  image: {
    width: '100%',
    height: 'auto',
    objectFit: 'cover' as const,
    display: 'block',
  } as React.CSSProperties,
  
  header: {
    padding: 'var(--spacing-4)',
    paddingBottom: 'var(--spacing-2)',
  } as React.CSSProperties,
  
  title: {
    fontSize: 'var(--text-heading-sm)',
    fontWeight: 'var(--font-weight-semibold)',
    color: 'var(--text-primary)',
    margin: 0,
    marginBottom: 'var(--spacing-1)',
  } as React.CSSProperties,
  
  subtitle: {
    fontSize: 'var(--text-body-sm)',
    color: 'var(--text-secondary)',
    margin: 0,
  } as React.CSSProperties,
  
  content: {
    padding: 'var(--spacing-4)',
  } as React.CSSProperties,
  
  footer: {
    padding: 'var(--spacing-3)',
    backgroundColor: 'var(--bg-secondary)',
    borderTop: '1px solid var(--border-primary)',
  } as React.CSSProperties,
};

// Named exports for different card variants
export const CardDefault = forwardRef<HTMLDivElement, Omit<CardProps, 'variant'>>(
  (props, ref) => <Card ref={ref} variant="default" {...props} />
);

CardDefault.displayName = 'CardDefault';

export const CardFlat = forwardRef<HTMLDivElement, Omit<CardProps, 'variant'>>(
  (props, ref) => <Card ref={ref} variant="flat" {...props} />
);

CardFlat.displayName = 'CardFlat';

export const CardHoverable = forwardRef<HTMLDivElement, Omit<CardProps, 'variant'>>(
  (props, ref) => <Card ref={ref} variant="hoverable" {...props} />
);

CardHoverable.displayName = 'CardHoverable';

export const CardInteractive = forwardRef<HTMLDivElement, Omit<CardProps, 'variant'>>(
  (props, ref) => <Card ref={ref} variant="interactive" {...props} />
);

CardInteractive.displayName = 'CardInteractive';

export { CardHeader, CardContent, CardFooter };
export default Card;