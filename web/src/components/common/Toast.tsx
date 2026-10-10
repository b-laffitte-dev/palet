import React, { createContext, useContext, useState, useCallback, useEffect, ReactNode } from 'react';
import { Icon, IconName } from './Icon';
import { cva, type VariantProps } from 'class-variance-authority';

// Toast variants
const toastVariants = cva(
  'fixed bottom-6 right-6 z-60 flex items-center gap-4 p-4 rounded-lg shadow-lg min-w-[300px] max-w-[400px]',
  {
    variants: {
      variant: {
        success: 'bg-success-500 text-white',
        error: 'bg-danger-500 text-white',
        warning: 'bg-warning-500 text-white',
        info: 'bg-info-500 text-white',
        primary: 'bg-primary-500 text-white',
        secondary: 'bg-secondary-500 text-white',
      },
      position: {
        'bottom-right': 'bottom-6 right-6',
        'bottom-left': 'bottom-6 left-6',
        'top-right': 'top-6 right-6',
        'top-left': 'top-6 left-6',
        'bottom-center': 'bottom-6 left-1/2 -translate-x-1/2',
        'top-center': 'top-6 left-1/2 -translate-x-1/2',
      },
    },
    defaultVariants: {
      variant: 'info',
      position: 'bottom-right',
    },
  }
);

// Toast types
export type ToastVariant = 'success' | 'error' | 'warning' | 'info' | 'primary' | 'secondary';
export type ToastPosition = 'bottom-right' | 'bottom-left' | 'top-right' | 'top-left' | 'bottom-center' | 'top-center';

// Toast props
export interface ToastProps extends VariantProps<typeof toastVariants> {
  id: string;
  title?: string;
  message: string;
  icon?: IconName;
  duration?: number;
  onClose?: (id: string) => void;
  action?: {
    label: string;
    onClick: () => void;
    icon?: IconName;
  };
  closeButton?: boolean;
  closeOnClick?: boolean;
  progressBar?: boolean;
}

// Toast component
const Toast: React.FC<ToastProps> = ({
  id,
  title,
  message,
  variant = 'info',
  position = 'bottom-right',
  icon,
  duration = 5000,
  onClose,
  action,
  closeButton = true,
  closeOnClick = true,
  progressBar = true,
}) => {
  const [progress, setProgress] = useState(0);
  const [isExiting, setIsExiting] = useState(false);

  // Start timer
  useEffect(() => {
    const timer = setTimeout(() => {
      setIsExiting(true);
      setTimeout(() => onClose?.(id), 300);
    }, duration);

    // Progress animation
    const progressInterval = setInterval(() => {
      setProgress(prev => {
        const newProgress = prev + (100 / (duration / 50));
        return newProgress >= 100 ? 100 : newProgress;
      });
    }, 50);

    return () => {
      clearTimeout(timer);
      clearInterval(progressInterval);
    };
  }, [duration, id, onClose]);

  const handleClose = () => {
    setIsExiting(true);
    setTimeout(() => onClose?.(id), 300);
  };

  const handleClick = () => {
    if (closeOnClick) {
      handleClose();
    }
  };

  const getIcon = (): IconName => {
    if (icon) return icon;
    switch (variant) {
      case 'success': return 'CheckCircle';
      case 'error': return 'AlertCircle';
      case 'warning': return 'AlertTriangle';
      case 'info': return 'Info';
      case 'primary': return 'Bell';
      case 'secondary': return 'Bell';
      default: return 'Bell';
    }
  };

  const styles = {
    entering: {
      opacity: 1,
      transform: 'translateY(0)',
      transition: 'opacity 300ms ease, transform 300ms ease',
    } as React.CSSProperties,
    exiting: {
      opacity: 0,
      transform: 'translateY(20px)',
      transition: 'opacity 300ms ease, transform 300ms ease',
    } as React.CSSProperties,
    progressBar: {
      height: '4px',
      background: 'rgba(255, 255, 255, 0.3)',
      borderRadius: '0 0 8px 8px',
      overflow: 'hidden',
      width: '100%',
    } as React.CSSProperties,
    progressFill: {
      height: '100%',
      background: 'rgba(255, 255, 255, 0.7)',
      borderRadius: '0 0 8px 8px',
      width: `${progress}%`,
      transition: 'width 50ms linear',
    } as React.CSSProperties,
  };

  return (
    <div
      className={toastVariants({ variant, position, className: isExiting ? 'opacity-0 translate-y-2 transition-all' : 'opacity-100 translate-y-0 transition-all' })}
      onClick={handleClick}
      style={isExiting ? styles.exiting : styles.entering}
    >
      <div className="flex-shrink-0">
        <Icon name={getIcon()} size="m" />
      </div>
      
      <div className="flex-1">
        {title && <h4 className="font-semibold text-sm mb-0.5">{title}</h4>}
        <p className="text-sm opacity-90">{message}</p>
      </div>
      
      {action && (
        <button
          type="button"
          onClick={(e) => {
            e.stopPropagation();
            action.onClick();
          }}
          className="flex-shrink-0 px-3 py-1.5 text-sm font-medium hover:bg-white/20 rounded transition-colors flex items-center gap-1"
        >
          {action.icon && <Icon name={action.icon} size="xs" />}
          {action.label}
        </button>
      )}
      
      {closeButton && (
        <button
          type="button"
          onClick={(e) => {
            e.stopPropagation();
            handleClose();
          }}
          className="flex-shrink-0 p-1 hover:bg-white/20 rounded transition-colors"
          aria-label="Fermer"
        >
          <Icon name="X" size="m" />
        </button>
      )}
      
      {progressBar && !isExiting && (
        <div style={styles.progressBar}>
          <div style={styles.progressFill} />
        </div>
      )}
    </div>
  );
};

// Toast context types
interface ToastContextType {
  toasts: ToastProps[];
  addToast: (toast: Omit<ToastProps, 'id' | 'onClose'>) => string;
  removeToast: (id: string) => void;
  clearToasts: () => void;
  updateToast: (id: string, updates: Partial<ToastProps>) => void;
}

// Create context
const ToastContext = createContext<ToastContextType | undefined>(undefined);

// Toast provider
export interface ToastProviderProps {
  children: ReactNode;
  position?: ToastPosition;
  maxToasts?: number;
}

export const ToastProvider: React.FC<ToastProviderProps> = ({
  children,
  position = 'bottom-right',
  maxToasts = 5,
}) => {
  const [toasts, setToasts] = useState<ToastProps[]>([]);

  const addToast = useCallback((toast: Omit<ToastProps, 'id' | 'onClose'>): string => {
    const id = Math.random().toString(36).substring(2) + Date.now().toString(36);
    setToasts(prev => {
      // Remove oldest if at max
      if (prev.length >= maxToasts) {
        return [...prev.slice(1), { ...toast, id, position, onClose: removeToast }];
      }
      return [...prev, { ...toast, id, position, onClose: removeToast }];
    });
    return id;
  }, [maxToasts, position]);

  const removeToast = useCallback((id: string) => {
    setToasts(prev => prev.filter(t => t.id !== id));
  }, []);

  const clearToasts = useCallback(() => {
    setToasts([]);
  }, []);

  const updateToast = useCallback((id: string, updates: Partial<ToastProps>) => {
    setToasts(prev => prev.map(t => t.id === id ? { ...t, ...updates } : t));
  }, []);

  const value = {
    toasts,
    addToast,
    removeToast,
    clearToasts,
    updateToast,
  };

  return (
    <ToastContext.Provider value={value}>
      {children}
      <div className="fixed inset-0 pointer-events-none z-60">
        {toasts.map(toast => (
          <Toast 
            key={toast.id} 
            {...toast}
            onClose={removeToast}
          />
        ))}
      </div>
    </ToastContext.Provider>
  );
};

// Toast hook
export function useToast(): ToastContextType {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error('useToast must be used within a ToastProvider');
  }
  return context;
}

// Toast component for easy usage
export interface CreateToastProps extends Omit<ToastProps, 'id' | 'onClose'> {
}

export function createToast(toast: Omit<ToastProps, 'id' | 'onClose'>): () => void {
  const id = Math.random().toString(36).substring(2) + Date.now().toString(36);
  const safePosition: ToastPosition = toast.position ? toast.position : 'bottom-right';
  const element = (
    <Toast
      {...toast}
      id={id}
      position={safePosition}
      onClose={() => {}}
    />
  );
  
  const container = document.createElement('div');
  container.className = 'fixed inset-0 pointer-events-none z-60';
  container.style.position = 'fixed';
  container.style.top = '0';
  container.style.left = '0';
  container.style.right = '0';
  container.style.bottom = '0';
  container.style.pointerEvents = 'none';
  container.style.zIndex = '60';
  
  document.body.appendChild(container);
  container.appendChild(element as unknown as Node);
  
  const remove = () => {
    document.body.removeChild(container);
  };
  
  return remove;
}

// Simple toast functions
export const toast = {
  success: (message: string, options?: Omit<CreateToastProps, 'message' | 'variant'>) => {
    const remove = createToast({ ...options, message, variant: 'success' });
    setTimeout(remove, options?.duration || 5000);
    return remove;
  },
  error: (message: string, options?: Omit<CreateToastProps, 'message' | 'variant'>) => {
    const remove = createToast({ ...options, message, variant: 'error' });
    setTimeout(remove, options?.duration || 5000);
    return remove;
  },
  warning: (message: string, options?: Omit<CreateToastProps, 'message' | 'variant'>) => {
    const remove = createToast({ ...options, message, variant: 'warning' });
    setTimeout(remove, options?.duration || 5000);
    return remove;
  },
  info: (message: string, options?: Omit<CreateToastProps, 'message' | 'variant'>) => {
    const remove = createToast({ ...options, message, variant: 'info' });
    setTimeout(remove, options?.duration || 5000);
    return remove;
  },
  primary: (message: string, options?: Omit<CreateToastProps, 'message' | 'variant'>) => {
    const remove = createToast({ ...options, message, variant: 'primary' });
    setTimeout(remove, options?.duration || 5000);
    return remove;
  },
  custom: (props: ToastProps) => {
    const remove = createToast({ ...props, variant: props.variant || 'info' });
    setTimeout(remove, props.duration || 5000);
    return remove;
  },
};

export default Toast;
