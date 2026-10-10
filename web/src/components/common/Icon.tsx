import * as LucideIcons from 'lucide-react';
import { LucideIcon as LucideIconType } from 'lucide-react';

export type IconName = keyof typeof LucideIcons;

export interface IconProps {
  name: IconName;
  size?: 'xxs' | 'xs' | 'sm' | 'm' | 'l' | 'xl' | 'xxl';
  color?: string;
  className?: string;
  onClick?: () => void;
  filled?: boolean;
  strokeWidth?: number;
}

// Map size names to actual sizes
const sizeMap: Record<Exclude<IconProps['size'], undefined>, number> = {
  xxs: 12,
  xs: 16,
  sm: 20,
  m: 24,
  l: 28,
  xl: 32,
  xxl: 40,
};

export function Icon({
  name,
  size = 'm',
  color = 'currentColor',
  className = '',
  onClick,
  filled = false,
  strokeWidth = 2,
}: IconProps) {
  const LucideIcon = LucideIcons[name] as LucideIconType | undefined;
  
  if (!LucideIcon) {
    console.warn(`Icon "${name}" not found in lucide-react`);
    return null;
  }

  const IconComponent = LucideIcon;
  return (
    <IconComponent
      size={sizeMap[size]}
      color={color}
      className={className}
      onClick={onClick}
      strokeWidth={filled ? 3 : strokeWidth}
      fill={filled ? color : 'none'}
      style={{
        display: 'inline-block',
        flexShrink: 0,
        cursor: onClick ? 'pointer' : 'default',
        ...(onClick && { transition: 'transform 0.2s ease' }),
      }}
    />
  );
}

export default Icon;