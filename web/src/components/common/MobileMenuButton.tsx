import { forwardRef, ButtonHTMLAttributes } from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { Icon } from "./Icon";

// Mobile menu button variants
const mobileMenuButtonVariants = cva(
  "flex items-center justify-center rounded-md transition-colors duration-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50",
  {
    variants: {
      variant: {
        default: "bg-transparent text-primary hover:bg-primary-800/50",
        outline: "border border-neutral-600 bg-transparent text-primary hover:bg-primary-800/50",
        solid: "bg-primary-700 text-white hover:bg-primary-600",
      },
      size: {
        sm: "p-2",
        m: "p-2.5",
        l: "p-3",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "m",
    },
  },
);

export interface MobileMenuButtonProps
  extends ButtonHTMLAttributes<HTMLButtonElement>, VariantProps<typeof mobileMenuButtonVariants> {
  isOpen?: boolean;
  label?: string;
}

const MobileMenuButton = forwardRef<HTMLButtonElement, MobileMenuButtonProps>(
  ({ className, variant, size, isOpen = false, label = "Menu", ...props }, ref) => {
    const baseStyles = mobileMenuButtonVariants({ variant, size });

    return (
      <button
        ref={ref}
        className={`${baseStyles} ${className || ""}`.trim()}
        aria-label={label}
        aria-expanded={isOpen}
        type="button"
        {...props}
      >
        {isOpen ? <Icon name="X" size={size || "m"} /> : <Icon name="Menu" size={size || "m"} />}
      </button>
    );
  },
);

MobileMenuButton.displayName = "MobileMenuButton";

export default MobileMenuButton;
