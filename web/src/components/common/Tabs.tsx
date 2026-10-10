import React, { forwardRef, useState, useCallback, ReactNode, Children, cloneElement, isValidElement } from 'react';
import { Icon, IconName } from './Icon';
import { cva, type VariantProps } from 'class-variance-authority';

// Tabs variants
const tabsContainerVariants = cva(
  'flex items-center',
  {
    variants: {
      variant: {
        default: 'gap-2',
        pills: 'gap-2 bg-primary-800 p-1 rounded-xl',
        underline: 'gap-8 border-b border-primary-700',
        segmented: 'gap-0 bg-primary-800 rounded-xl p-1',
      },
      size: {
        xs: 'text-xs',
        sm: 'text-sm',
        m: 'text-base',
        l: 'text-lg',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'm',
    },
  }
);

// Tab variants
const tabVariants = cva(
  'flex items-center gap-2 px-4 py-2 font-medium transition-all focus:outline-none focus-visible:ring-2',
  {
    variants: {
      variant: {
        default: '',
        pills: 'rounded-lg',
        underline: 'pb-3 border-b-2',
        segmented: 'rounded-lg',
      },
      state: {
        active: 'text-white bg-primary-700',
        inactive: 'text-primary-300 hover:text-white hover:bg-primary-700/50',
      },
      size: {
        xs: 'px-2 py-1 text-xs',
        sm: 'px-3 py-1.5 text-sm',
        m: 'px-4 py-2 text-base',
        l: 'px-6 py-3 text-lg',
      },
    },
    compoundVariants: [
      {
        variant: 'underline',
        state: 'active',
        className: 'border-primary-500',
      },
      {
        variant: 'underline',
        state: 'inactive',
        className: 'border-transparent hover:border-primary-500',
      },
      {
        variant: 'pills',
        state: 'active',
        className: 'bg-white text-primary-900',
      },
      {
        variant: 'segmented',
        state: 'active',
        className: 'bg-primary-600 text-white shadow-sm',
      },
      {
        variant: 'segmented',
        state: 'inactive',
        className: 'text-primary-300 hover:bg-primary-700/50',
      },
    ],
    defaultVariants: {
      variant: 'default',
      state: 'inactive',
      size: 'm',
    },
  }
);

// Tab types
type TabSize = 'xs' | 'sm' | 'm' | 'l';

// Tab props
export interface TabProps extends VariantProps<typeof tabVariants> {
  value: string;
  label?: string;
  icon?: IconName;
  disabled?: boolean;
  badge?: string | number;
  onClick?: (value: string) => void;
  className?: string;
}

// Tab component
const Tab = forwardRef<HTMLButtonElement, TabProps>(
  (
    {
      value,
      label,
      icon,
      disabled,
      badge,
      onClick,
      variant = 'default',
      size = 'm',
      state = 'inactive',
      className,
      ...props
    },
    ref
  ) => {
    const handleClick = () => {
      if (!disabled && onClick) {
        onClick(value);
      }
    };

    return (
      <button
        ref={ref}
        type="button"
        onClick={handleClick}
        disabled={disabled}
        className={tabVariants({ variant, state, size, className })}
        aria-selected={state === 'active'}
        aria-disabled={disabled}
        {...props}
      >
        {icon && <Icon name={icon} size={size as TabSize} />}
        {label && <span>{label}</span>}
        {badge && (
          <span className="px-2 py-0.5 bg-gold-500 text-gold-900 text-xs font-bold rounded-full">
            {badge}
          </span>
        )}
      </button>
    );
  }
);

Tab.displayName = 'Tab';

// Tabs props
export interface TabsProps extends VariantProps<typeof tabsContainerVariants> {
  children: ReactNode;
  defaultValue?: string;
  value?: string;
  onChange?: (value: string) => void;
  orientation?: 'horizontal' | 'vertical';
  className?: string;
  tabClassName?: string;
  disabled?: boolean;
}

// Tabs component
const Tabs = forwardRef<HTMLDivElement, TabsProps>(
  (
    {
      children,
      defaultValue,
      value: controlledValue,
      onChange,
      variant = 'default',
      size = 'm',
      orientation = 'horizontal',
      className,
      tabClassName,
      disabled = false,
      ...props
    },
    ref
  ) => {
    const [internalValue, setInternalValue] = useState(defaultValue || '');
    const isControlled = controlledValue !== undefined;
    const currentValue = isControlled ? controlledValue : internalValue;

    const handleTabChange = useCallback((tabValue: string) => {
      if (disabled) return;
      
      if (!isControlled) {
        setInternalValue(tabValue);
      }
      onChange?.(tabValue);
    }, [disabled, isControlled, onChange]);

    const validChildren = Children.toArray(children).filter(child => 
      isValidElement(child) && (child as React.ReactElement).type === Tab
    );

    // Clone children and inject props
    const tabs = validChildren.map(child => {
      if (isValidElement(child)) {
        const childElement = child as React.ReactElement<TabProps>;
        const isActive = childElement.props.value === currentValue;
        return cloneElement(childElement, {
          variant,
          size,
          state: isActive ? 'active' : 'inactive',
          onClick: handleTabChange,
          className: `${tabClassName || ''} ${childElement.props.className || ''}`,
        });
      }
      return child;
    });

    // Find active tab content
    let activeContent: ReactNode | null = null;
    Children.forEach(children, child => {
      if (isValidElement(child)) {
        const childElement = child as React.ReactElement<{ value?: string; children?: ReactNode }>;
        if (childElement.props.value === currentValue) {
          activeContent = childElement.props.children;
        }
      }
    });

    const containerStyles = {
      horizontal: 'flex-row' as const,
      vertical: 'flex-col' as const,
    };

    const contentContainerStyles = {
      horizontal: 'mt-4',
      vertical: 'ml-4',
    };

    return (
      <div ref={ref} className={`${className || ''}`} {...props}>
        <div className={tabsContainerVariants({ variant, size })} style={{ flexDirection: containerStyles[orientation] as any }}>
          {tabs}
        </div>
        
        {/* Tab content */}
        {activeContent && (
          <div style={{ flex: 1 }} className={contentContainerStyles[orientation]}>
            {activeContent}
          </div>
        )}
      </div>
    );
  }
);

Tabs.displayName = 'Tabs';

// Tab Panel for accessibility
export interface TabPanelProps {
  value: string;
  children: ReactNode;
  className?: string;
}

export const TabPanel = forwardRef<HTMLDivElement, TabPanelProps>(
  ({ value, children, className, ...props }, ref) => {
    // This is handled by the parent Tabs component
    return (
      <div ref={ref} className={className} {...props}>
        {children}
      </div>
    );
  }
);

TabPanel.displayName = 'TabPanel';

// Tab List wrapper
export interface TabListProps {
  children: ReactNode;
  variant?: TabsProps['variant'];
  size?: TabsProps['size'];
  className?: string;
}

export const TabList = forwardRef<HTMLDivElement, TabListProps>(
  ({ children, variant = 'default', size = 'm', className, ...props }, ref) => {
    return (
      <div 
        ref={ref}
        className={tabsContainerVariants({ variant, size, className })}
        role="tablist"
        {...props}
      >
        {Children.map(children, child => {
          if (isValidElement(child)) {
            const childElement = child as React.ReactElement<TabProps>;
            return cloneElement(childElement, {
              variant,
              size,
            });
          }
          return child;
        })}
      </div>
    );
  }
);

TabList.displayName = 'TabList';

export { Tab };
export default Tabs;
