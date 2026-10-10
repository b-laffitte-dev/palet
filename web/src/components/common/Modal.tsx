import React, { forwardRef, ReactNode, useEffect, useCallback, useRef, useState } from 'react';
import { Icon, IconName } from './Icon';
import Button, { ButtonProps } from './Button';
import { cva, type VariantProps } from 'class-variance-authority';

// Modal variants
const modalVariants = cva(
  'fixed inset-0 z-50 flex items-center justify-center p-4',
  {
    variants: {
      size: {
        xs: 'max-w-xs',
        sm: 'max-w-sm',
        m: 'max-w-md',
        l: 'max-w-lg',
        xl: 'max-w-xl',
        '2xl': 'max-w-2xl',
        '3xl': 'max-w-3xl',
        full: 'max-w-[90vw] max-h-[90vh]',
      },
    },
    defaultVariants: {
      size: 'm',
    },
  }
);

// Modal content variants
const modalContentVariants = cva(
  'bg-primary-800 rounded-xl shadow-2xl overflow-hidden',
  {
    variants: {
      size: {
        xs: 'w-full',
        sm: 'w-full',
        m: 'w-full',
        l: 'w-full',
        xl: 'w-full',
        '2xl': 'w-full',
        '3xl': 'w-full',
        full: 'w-full max-h-[80vh]',
      },
    },
  }
);

// Modal props
export interface ModalProps extends VariantProps<typeof modalVariants> {
  isOpen: boolean;
  onClose?: () => void;
  onOpen?: () => void;
  children: ReactNode;
  title?: string;
  description?: string;
  closeOnOverlayClick?: boolean;
  closeOnEscape?: boolean;
  showCloseButton?: boolean;
  closeIcon?: IconName;
  closeButtonVariant?: ButtonProps['variant'];
  size?: 'xs' | 'sm' | 'm' | 'l' | 'xl' | '2xl' | '3xl' | 'full';
  className?: string;
  contentClassName?: string;
  overlayClassName?: string;
  trapFocus?: boolean;
}

// Modal component
const Modal = forwardRef<HTMLDivElement, ModalProps>(
  (
    {
      isOpen,
      onClose,
      onOpen,
      children,
      title,
      description,
      closeOnOverlayClick = true,
      closeOnEscape = true,
      showCloseButton = true,
      closeIcon = 'X',
      closeButtonVariant = 'ghost',
      size = 'm',
      className,
      contentClassName,
      overlayClassName,
      trapFocus = true,
      ...props
    },
    ref
  ) => {
    const modalRef = useRef<HTMLDivElement>(null);
    const previousActiveElement = useRef<HTMLElement | null>(null);
    const [isMounted, setIsMounted] = useState(false);

    // Track if modal is open
    useEffect(() => {
      if (isOpen) {
        onOpen?.();
        setIsMounted(true);
        previousActiveElement.current = document.activeElement as HTMLElement;
        
        // Add body overflow hidden
        document.body.style.overflow = 'hidden';
        
        // Focus trap
        if (trapFocus && modalRef.current) {
          modalRef.current.focus();
        }
      } else {
        // Restore body overflow
        document.body.style.overflow = '';
        
        // Focus back to previous element
        if (previousActiveElement.current) {
          previousActiveElement.current.focus();
        }
      }
    }, [isOpen, onOpen, trapFocus]);

    // Cleanup on unmount
    useEffect(() => {
      return () => {
        document.body.style.overflow = '';
      };
    }, []);

    // Close on escape
    const handleKeyDown = useCallback((event: KeyboardEvent) => {
      if (closeOnEscape && event.key === 'Escape' && isOpen) {
        onClose?.();
      }
    }, [closeOnEscape, isOpen, onClose]);

    useEffect(() => {
      if (isOpen) {
        document.addEventListener('keydown', handleKeyDown);
      }
      return () => {
        document.removeEventListener('keydown', handleKeyDown);
      };
    }, [isOpen, handleKeyDown]);

    // Close on overlay click
    const handleOverlayClick = useCallback((event: React.MouseEvent<HTMLDivElement>) => {
      if (closeOnOverlayClick && event.target === event.currentTarget && isOpen) {
        onClose?.();
      }
    }, [closeOnOverlayClick, isOpen, onClose]);

    // Focus trap
    const handleKeyDownFocus = useCallback((event: React.KeyboardEvent<HTMLDivElement>) => {
      if (trapFocus && event.key === 'Tab' && modalRef.current) {
        const focusableElements = modalRef.current.querySelectorAll<HTMLElement>(
          'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
        );
        const firstElement = focusableElements[0];
        const lastElement = focusableElements[focusableElements.length - 1];
        
        if (event.shiftKey) {
          // Shift + Tab: moving backwards
          if (focusableElements.length > 0 && document.activeElement === firstElement) {
            event.preventDefault();
            lastElement?.focus();
          }
        } else {
          // Tab: moving forwards
          if (focusableElements.length > 0 && document.activeElement === lastElement) {
            event.preventDefault();
            firstElement?.focus();
          }
        }
      }
    }, [trapFocus]);

    // Animation styles
    const getOverlayStyle = (): React.CSSProperties => ({
      opacity: isOpen && isMounted ? 1 : 0,
      transition: 'opacity 200ms ease',
    });

    const getContentStyle = (): React.CSSProperties => ({
      opacity: isOpen && isMounted ? 1 : 0,
      transform: isOpen && isMounted ? 'translateY(0)' : 'translateY(-20px)',
      transition: 'opacity 200ms ease, transform 200ms ease',
    });

    if (!isMounted) return null;

    return (
      <div
        ref={ref}
        className={modalVariants({ size, className })}
        onClick={handleOverlayClick}
        style={getOverlayStyle()}
        {...props}
      >
        {/* Overlay */}
        <div className={`fixed inset-0 bg-black/50 backdrop-blur-sm ${overlayClassName || ''}`} />
        
        {/* Modal content */}
        <div
          ref={modalRef}
          className={modalContentVariants({ size, className: contentClassName })}
          style={getContentStyle()}
          onKeyDown={handleKeyDownFocus}
          tabIndex={-1}
        >
          {/* Header */}
          {(title || showCloseButton) && (
            <div className="flex items-center justify-between p-6 border-b border-primary-700">
              <div className="flex-1">
                {title && (
                  <h2 className="text-h4 font-bold text-white mb-1">{title}</h2>
                )}
                {description && (
                  <p className="text-body-s text-primary-300">{description}</p>
                )}
              </div>
              
              {showCloseButton && onClose && (
                <Button
                  variant={closeButtonVariant}
                  size="sm"
                  onClick={onClose}
                  aria-label="Fermer"
                  className="flex-shrink-0"
                  leftIcon={closeIcon === 'X' ? undefined : closeIcon}
                >
                  {closeIcon === 'X' && <Icon name="X" size="m" />}
                </Button>
              )}
            </div>
          )}
          
          {/* Body */}
          <div className="p-6">
            {children}
          </div>
        </div>
      </div>
    );
  }
);

Modal.displayName = 'Modal';

// Modal Header component
export interface ModalHeaderProps {
  children: ReactNode;
  className?: string;
}

export const ModalHeader = forwardRef<HTMLDivElement, ModalHeaderProps>(
  ({ children, className, ...props }, ref) => (
    <div 
      ref={ref} 
      className={`p-6 border-b border-primary-700 ${className || ''}`}
      {...props}
    >
      {children}
    </div>
  )
);

ModalHeader.displayName = 'ModalHeader';

// Modal Body component
export interface ModalBodyProps {
  children: ReactNode;
  className?: string;
}

export const ModalBody = forwardRef<HTMLDivElement, ModalBodyProps>(
  ({ children, className, ...props }, ref) => (
    <div 
      ref={ref} 
      className={`p-6 ${className || ''}`}
      {...props}
    >
      {children}
    </div>
  )
);

ModalBody.displayName = 'ModalBody';

// Modal Footer component
export interface ModalFooterProps {
  children: ReactNode;
  className?: string;
}

export const ModalFooter = forwardRef<HTMLDivElement, ModalFooterProps>(
  ({ children, className, ...props }, ref) => (
    <div 
      ref={ref} 
      className={`p-6 border-t border-primary-700 flex items-center justify-end gap-3 ${className || ''}`}
      {...props}
    >
      {children}
    </div>
  )
);

ModalFooter.displayName = 'ModalFooter';

// Convenience hooks
export function useModal() {
  const [isOpen, setIsOpen] = useState(false);
  
  const open = useCallback(() => setIsOpen(true), []);
  const close = useCallback(() => setIsOpen(false), []);
  const toggle = useCallback(() => setIsOpen(prev => !prev), []);
  
  return { isOpen, open, close, toggle, setIsOpen };
}

// Modal wrapper for simple usage
interface SimpleModalProps extends Omit<ModalProps, 'isOpen' | 'children'> {
  trigger: ReactNode;
  children: ReactNode | ((close: () => void) => ReactNode);
  closeOnTriggerClick?: boolean;
  onClose?: () => void;
}

export const SimpleModal = forwardRef<HTMLDivElement, SimpleModalProps>(
  (
    {
      trigger,
      children,
      closeOnTriggerClick = true,
      onOpen,
      onClose: onCloseProp,
      ...props
    },
    ref
  ) => {
    const { isOpen, open, close, setIsOpen } = useModal();
    
    const handleClose = () => {
      close();
      onCloseProp?.();
    };

    const handleTriggerClick = () => {
      if (closeOnTriggerClick && isOpen) {
        handleClose();
      } else {
        open();
      }
    };

    return (
      <>
        <div onClick={handleTriggerClick}>
          {trigger}
        </div>
        <Modal
          ref={ref}
          isOpen={isOpen}
          onClose={handleClose}
          onOpen={onOpen}
          {...props}
        >
          {typeof children === 'function' ? children(handleClose) : children}
        </Modal>
      </>
    );
  }
);

SimpleModal.displayName = 'SimpleModal';

export default Modal;
