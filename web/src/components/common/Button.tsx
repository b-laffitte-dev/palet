import { forwardRef, ButtonHTMLAttributes, ReactNode } from "react";
import { Icon, IconName } from "./Icon";
import { cva, type VariantProps } from "class-variance-authority";

// Button variants using CVA (Class Variance Authority pattern)
const buttonVariants = cva(
  "inline-flex items-center justify-center font-medium transition-all focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed rounded-md",
  {
    variants: {
      variant: {
        primary: "bg-brand-green-600 text-white hover:bg-brand-green-700 active:bg-brand-green-800",
        secondary: "bg-neutral-100 text-neutral-900 hover:bg-neutral-200 active:bg-neutral-300",
        outline:
          "border border-neutral-300 bg-transparent text-neutral-700 hover:bg-neutral-50 active:bg-neutral-100",
        ghost: "bg-transparent text-neutral-600 hover:bg-neutral-100 active:bg-neutral-200",
        gold: "bg-brand-orange-600 text-white hover:bg-brand-orange-700 active:bg-brand-orange-800",
        danger: "bg-danger-600 text-white hover:bg-danger-700 active:bg-danger-800",
        link: "bg-transparent text-brand-green-600 hover:text-brand-green-700 hover:underline p-0",
      },
      size: {
        xs: "px-2 py-1 text-xs gap-1",
        sm: "px-3 py-1.5 text-sm gap-1.5",
        m: "px-4 py-2 text-sm gap-2",
        l: "px-6 py-3 text-base gap-2.5",
        xl: "px-8 py-4 text-lg gap-3",
      },
      shape: {
        square: "aspect-square",
        round: "rounded-full",
        default: "rounded-md",
      },
      fullWidth: {
        true: "w-full",
        false: "",
      },
    },
    compoundVariants: [
      // Primary button with gold accent
      {
        variant: "primary",
        className: "shadow-sm hover:shadow-md active:shadow-none",
      },
      // Brand orange button with special styling
      {
        variant: "gold",
        className: "shadow-orange-sm hover:shadow-orange-m active:shadow-orange-l",
      },
      // Ghost button on dark background
      {
        variant: "ghost",
        className: "hover:text-neutral-900",
      },
      // Responsive full width on mobile
      {
        fullWidth: true,
        size: "m",
        className: "sm:w-auto",
      },
    ],
    defaultVariants: {
      variant: "primary",
      size: "m",
      shape: "default",
      fullWidth: false,
    },
  },
);

// Button props
export interface ButtonProps
  extends ButtonHTMLAttributes<HTMLButtonElement>, VariantProps<typeof buttonVariants> {
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
    ref,
  ) => {
    // Combine custom styles with variant classes
    const baseStyles = buttonVariants({ variant, size, shape, fullWidth });

    // Responsive classes
    const responsiveClasses = [
      "sm:min-w-[100px]",
      "transition-all",
      "duration-200",
      "ease-in-out",
    ].join(" ");

    return (
      <button
        ref={ref}
        className={`${baseStyles} ${responsiveClasses} ${className || ""}`.trim()}
        disabled={disabled || loading}
        {...props}
      >
        {loading ? (
          <span className="animate-spin flex items-center justify-center">
            <Icon name="Loader2" size="sm" />
          </span>
        ) : (
          <>
            {leftIcon && <Icon name={leftIcon} size={size || "m"} />}
            {children && !iconOnly && <span className={iconOnly ? "sr-only" : ""}>{children}</span>}
            {rightIcon && <Icon name={rightIcon} size={size || "m"} />}
          </>
        )}
      </button>
    );
  },
);

Button.displayName = "Button";

// Named exports for different button variants
export const ButtonPrimary = forwardRef<HTMLButtonElement, Omit<ButtonProps, "variant">>(
  (props, ref) => <Button ref={ref} variant="primary" {...props} />,
);

ButtonPrimary.displayName = "ButtonPrimary";

export const ButtonSecondary = forwardRef<HTMLButtonElement, Omit<ButtonProps, "variant">>(
  (props, ref) => <Button ref={ref} variant="secondary" {...props} />,
);

ButtonSecondary.displayName = "ButtonSecondary";

export const ButtonOutline = forwardRef<HTMLButtonElement, Omit<ButtonProps, "variant">>(
  (props, ref) => <Button ref={ref} variant="outline" {...props} />,
);

ButtonOutline.displayName = "ButtonOutline";

export const ButtonGhost = forwardRef<HTMLButtonElement, Omit<ButtonProps, "variant">>(
  (props, ref) => <Button ref={ref} variant="ghost" {...props} />,
);

ButtonGhost.displayName = "ButtonGhost";

export const ButtonGold = forwardRef<HTMLButtonElement, Omit<ButtonProps, "variant">>(
  (props, ref) => <Button ref={ref} variant="gold" {...props} />,
);

ButtonGold.displayName = "ButtonGold";

export const ButtonDanger = forwardRef<HTMLButtonElement, Omit<ButtonProps, "variant">>(
  (props, ref) => <Button ref={ref} variant="danger" {...props} />,
);

ButtonDanger.displayName = "ButtonDanger";

export const ButtonLink = forwardRef<HTMLButtonElement, Omit<ButtonProps, "variant">>(
  (props, ref) => <Button ref={ref} variant="link" {...props} />,
);

ButtonLink.displayName = "ButtonLink";

export default Button;
