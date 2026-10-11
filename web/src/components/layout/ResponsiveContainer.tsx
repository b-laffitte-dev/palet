import { ReactNode } from "react";

/**
 * ResponsiveContainer - A wrapper component that ensures proper responsive behavior
 * Handles padding, max-width, and other responsive layout properties
 */
export interface ResponsiveContainerProps {
  children: ReactNode;
  className?: string;
  as?: "div" | "main" | "section" | "article" | "header" | "footer";
  noPadding?: boolean;
  fullWidth?: boolean;
}

export function ResponsiveContainer({
  children,
  className = "",
  as: Component = "div",
  noPadding = false,
  fullWidth = false,
}: ResponsiveContainerProps) {
  const baseClasses = [
    "w-full",
    fullWidth ? "max-w-none" : "max-w-[var(--container-max)]",
    "mx-auto",
  ];

  const paddingClasses = noPadding
    ? []
    : [
        "px-[var(--container-padding)]",
        "py-[var(--spacing-4)]",
        "sm:py-[var(--spacing-6)]",
        "lg:py-[var(--spacing-8)]",
      ];

  const allClasses = [...baseClasses, ...paddingClasses, className].join(" ");

  return <Component className={allClasses.trim()}>{children}</Component>;
}

export default ResponsiveContainer;
