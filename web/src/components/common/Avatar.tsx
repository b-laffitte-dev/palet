import React, { forwardRef, ImgHTMLAttributes, HTMLAttributes, useState } from 'react';
import { Icon, IconName } from './Icon';
import { Badge } from './Badge';
import { cva, type VariantProps } from 'class-variance-authority';

// Avatar variants
const avatarVariants = cva(
  'inline-flex items-center justify-center rounded-full overflow-hidden flex-shrink-0',
  {
    variants: {
      size: {
        '2xs': 'w-5 h-5 text-xs',
        xs: 'w-6 h-6 text-xs',
        sm: 'w-8 h-8 text-sm',
        m: 'w-10 h-10 text-base',
        l: 'w-12 h-12 text-lg',
        xl: 'w-14 h-14 text-xl',
        '2xl': 'w-16 h-16 text-2xl',
        '3xl': 'w-20 h-20 text-3xl',
      },
      variant: {
        default: 'bg-primary-700 text-white',
        primary: 'bg-primary-600 text-white',
        secondary: 'bg-secondary-600 text-white',
        success: 'bg-success-600 text-white',
        warning: 'bg-warning-600 text-white',
        danger: 'bg-danger-600 text-white',
        info: 'bg-info-600 text-white',
        gold: 'bg-gold-600 text-white',
        neutral: 'bg-neutral-600 text-white',
      },
      shape: {
        circle: 'rounded-full',
        square: 'rounded-lg',
      },
    },
    defaultVariants: {
      size: 'm',
      variant: 'default',
      shape: 'circle',
    },
  }
);

// Avatar size type
type AvatarSize = '2xs' | 'xs' | 'sm' | 'm' | 'l' | 'xl' | '2xl' | '3xl';
type IconSize = 'xs' | 'sm' | 'm' | 'l' | 'xl';

// Badge variant type
type BadgeVariant = 'default' | 'primary' | 'secondary' | 'success' | 'warning' | 'danger' | 'info' | 'gold' | 'subtle';

// Avatar props - extend ImgHTMLAttributes without conflicting properties
interface BaseAvatarProps {
  name?: string;
  initials?: string;
  icon?: IconName;
  placeholder?: React.ReactNode;
  badge?: string | number | React.ReactNode;
  badgePosition?: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left';
  badgeVariant?: BadgeVariant;
  status?: 'online' | 'offline' | 'busy' | 'away' | null;
  statusPosition?: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left';
  loading?: boolean;
  fallback?: string | React.ReactNode;
}

export interface AvatarProps extends 
  Omit<ImgHTMLAttributes<HTMLImageElement>, 'alt' | 'src' | 'loading'>,
  Omit<VariantProps<typeof avatarVariants>, 'src' | 'alt'>,
  BaseAvatarProps {
  src?: string;
  alt?: string;
}

// Avatar component
const Avatar = forwardRef<HTMLImageElement, AvatarProps>(
  (
    {
      src,
      alt = '',
      name,
      initials,
      icon,
      placeholder,
      badge,
      badgePosition = 'bottom-right',
      badgeVariant = 'gold',
      status = null,
      statusPosition = 'bottom-right',
      size = 'm',
      variant = 'default',
      shape = 'circle',
      loading = false,
      fallback,
      className,
      ...props
    },
    ref
  ) => {
    const [imageError, setImageError] = useState(false);
    const [imageLoaded, setImageLoaded] = useState(false);

    // Generate initials from name if not provided
    const computedInitials = initials || (name ? getInitials(name) : undefined);
    const showPlaceholder = (!src || imageError || loading) && !imageLoaded;

    const handleImageError = () => {
      setImageError(true);
    };

    const handleImageLoad = () => {
      setImageLoaded(true);
    };

    // Get border radius based on shape
    const getBorderRadius = () => {
      if (shape === 'circle') {
        return '9999px';
      }
      switch (size) {
        case '2xs': return '4px';
        case 'xs': return '4px';
        case 'sm': return '6px';
        case 'm': return '8px';
        case 'l': return '8px';
        case 'xl': return '10px';
        case '2xl': return '12px';
        case '3xl': return '12px';
        default: return '8px';
      }
    };

    // Get size in pixels for badge positioning
    const getSizeInPx = () => {
      switch (size) {
        case '2xs': return 20;
        case 'xs': return 24;
        case 'sm': return 32;
        case 'm': return 40;
        case 'l': return 48;
        case 'xl': return 56;
        case '2xl': return 64;
        case '3xl': return 80;
        default: return 40;
      }
    };

    // Calculate badge position
    const getBadgePosition = () => {
      const sizePx = getSizeInPx();
      const badgeSize = sizePx * 0.35;
      const offset = badgeSize * 0.2;

      switch (badgePosition) {
        case 'top-right':
          return {
            top: `${-offset}px`,
            right: `${-offset}px`,
          };
        case 'top-left':
          return {
            top: `${-offset}px`,
            left: `${-offset}px`,
          };
        case 'bottom-right':
          return {
            bottom: `${-offset}px`,
            right: `${-offset}px`,
          };
        case 'bottom-left':
          return {
            bottom: `${-offset}px`,
            left: `${-offset}px`,
          };
        default:
          return {
            bottom: `${-offset}px`,
            right: `${-offset}px`,
          };
      }
    };

    // Calculate status indicator position
    const getStatusPosition = () => {
      const sizePx = getSizeInPx();
      const indicatorSize = sizePx * 0.25;

      switch (statusPosition) {
        case 'top-right':
          return {
            top: '0',
            right: '0',
            transform: `translate(${indicatorSize / 2}px, ${-indicatorSize / 2}px)`,
          };
        case 'top-left':
          return {
            top: '0',
            left: '0',
            transform: `translate(${-indicatorSize / 2}px, ${-indicatorSize / 2}px)`,
          };
        case 'bottom-right':
          return {
            bottom: '0',
            right: '0',
            transform: `translate(${indicatorSize / 2}px, ${indicatorSize / 2}px)`,
          };
        case 'bottom-left':
          return {
            bottom: '0',
            left: '0',
            transform: `translate(${-indicatorSize / 2}px, ${indicatorSize / 2}px)`,
          };
        default:
          return {
            bottom: '0',
            right: '0',
            transform: `translate(${indicatorSize / 2}px, ${indicatorSize / 2}px)`,
          };
      }
    };

    // Get status color
    const getStatusColor = () => {
      switch (status) {
        case 'online': return 'var(--color-success-500)';
        case 'busy': return 'var(--color-danger-500)';
        case 'away': return 'var(--color-warning-500)';
        case 'offline': return 'var(--color-neutral-500)';
        default: return 'var(--color-neutral-500)';
      }
    };

    // Map size to icon size
    const safeSize = size || 'm';
    const iconSize: IconSize = safeSize === '2xs' || safeSize === 'xs' ? 'xs' : safeSize === 'xl' || safeSize === '2xl' || safeSize === '3xl' ? 'xl' : 'm';

    const baseClasses = avatarVariants({ size: size as AvatarSize, variant, shape, className });

    return (
      <div 
        className="relative inline-flex"
        style={{ width: getSizeInPx(), height: getSizeInPx() }}
      >
        {/* Avatar content */}
        {loading && !imageLoaded ? (
          <div
            className={`${baseClasses} bg-primary-800`}
            style={{ borderRadius: getBorderRadius() }}
          >
            <div className="animate-spin">
              <Icon name="Loader2" size={iconSize} />
            </div>
          </div>
        ) : showPlaceholder ? (
          <div
            className={`${baseClasses} ${!fallback ? variant : ''}`}
            style={{ borderRadius: getBorderRadius() }}
          >
            {fallback ? (
              typeof fallback === 'string' ? (
                <span className="font-semibold">{fallback}</span>
              ) : (
                fallback
              )
            ) : icon ? (
              <Icon name={icon} size={iconSize} />
            ) : placeholder ? (
              placeholder
            ) : computedInitials ? (
              <span className="font-semibold">{computedInitials}</span>
            ) : (
              <Icon name="User" size={iconSize} />
            )}
          </div>
        ) : (
          <img
            ref={ref}
            src={src}
            alt={alt}
            className={baseClasses}
            onError={handleImageError}
            onLoad={handleImageLoad}
            style={{ borderRadius: getBorderRadius(), objectFit: 'cover' }}
            loading="lazy"
            {...props}
          />
        )}

        {/* Status indicator */}
        {status && (
          <div
            className="absolute block rounded-full border-2 border-primary-800"
            style={{
              ...getStatusPosition(),
              width: `${getSizeInPx() * 0.25}px`,
              height: `${getSizeInPx() * 0.25}px`,
              backgroundColor: getStatusColor(),
              borderColor: 'var(--color-primary-800)',
            }}
          />
        )}

        {/* Badge */}
        {badge && (
          <div
            className="absolute flex items-center justify-center"
            style={getBadgePosition()}
          >
            {typeof badge === 'string' || typeof badge === 'number' ? (
              <Badge variant={badgeVariant as any} size="xs">
                {badge}
              </Badge>
            ) : (
              badge
            )}
          </div>
        )}
      </div>
    );
  }
);

Avatar.displayName = 'Avatar';

// Helper function to extract initials from name
function getInitials(name: string): string {
  const parts = name.trim().split(/\s+/).filter(Boolean);
  if (parts.length === 0) return '?';
  if (parts.length === 1) {
    return parts[0]!.charAt(0).toUpperCase();
  }
  return (parts[0]!.charAt(0) + parts[parts.length - 1]!.charAt(0)).toUpperCase();
}

// Avatar Group component
export interface AvatarGroupProps extends HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
  max?: number;
  spacing?: 'none' | 'xs' | 'sm' | 'm' | 'l';
  direction?: 'horizontal' | 'vertical';
}

export const AvatarGroup = forwardRef<HTMLDivElement, AvatarGroupProps>(
  (
    {
      children,
      max = 5,
      spacing = 'sm',
      direction = 'horizontal',
      className,
      ...props
    },
    ref
  ) => {
    const validChildren = React.Children.toArray(children);
    const visibleCount = Math.min(validChildren.length, max);
    const hiddenCount = validChildren.length - visibleCount;

    const spacingClasses = {
      none: '',
      xs: '-space-x-1',
      sm: '-space-x-2',
      m: '-space-x-3',
      l: '-space-x-4',
    };

    const directionClasses = {
      horizontal: `flex-row ${spacingClasses[spacing]}`,
      vertical: 'flex-col space-y-2',
    };

    return (
      <div 
        ref={ref}
        className={`flex ${directionClasses[direction]} ${className || ''}`}
        {...props}
      >
        {validChildren.slice(0, visibleCount).map((child, index) => (
          <div key={index} className={direction === 'horizontal' ? 'first:ml-0' : ''}>
            {child}
          </div>
        ))}
        
        {hiddenCount > 0 && (
          <div className={direction === 'horizontal' ? 'pl-2' : 'pt-2'}>
            <Avatar
              size={'m' as AvatarSize}
              variant="neutral"
              badge={hiddenCount > 99 ? '99+' : `+${hiddenCount}`}
              badgePosition="bottom-right"
              badgeVariant="primary"
            />
          </div>
        )}
      </div>
    );
  }
);

AvatarGroup.displayName = 'AvatarGroup';

// Stacked Avatars
export interface StackedAvatarsProps extends Omit<AvatarGroupProps, 'children'> {
  avatars: { src?: string; alt?: string; name?: string; [key: string]: any }[];
  max?: number;
  size?: AvatarSize;
}

export const StackedAvatars = forwardRef<HTMLDivElement, StackedAvatarsProps>(
  (
    {
      avatars,
      max = 3,
      size = 'm',
      ...props
    },
    ref
  ) => {
    const visibleCount = Math.min(avatars.length, max);
    const hiddenCount = avatars.length - visibleCount;

    return (
      <AvatarGroup ref={ref} max={max} {...props}>
        {avatars.slice(0, visibleCount).map((avatar, index) => (
          <Avatar
            key={index}
            src={avatar.src}
            alt={avatar.alt || avatar.name}
            name={avatar.name}
            size={size}
          />
        ))}
        
        {hiddenCount > 0 && (
          <Avatar
            size={size}
            variant="neutral"
            badge={`+${hiddenCount}`}
            badgePosition="bottom-right"
            badgeVariant="primary"
          />
        )}
      </AvatarGroup>
    );
  }
);

StackedAvatars.displayName = 'StackedAvatars';

// User Avatar with name and description
export interface UserAvatarProps extends Omit<AvatarProps, 'name' | 'initials' | 'alt' | 'src'> {
  user: {
    id?: string;
    name?: string;
    email?: string;
    photo?: string;
    initials?: string;
    [key: string]: any;
  };
  showName?: boolean;
  nameClassName?: string;
  showDescription?: boolean;
  description?: string;
  descriptionClassName?: string;
  direction?: 'horizontal' | 'vertical';
  gap?: 'none' | 'xs' | 'sm' | 'm' | 'l';
  size?: AvatarSize;
}

export const UserAvatar = forwardRef<HTMLDivElement, UserAvatarProps>(
  (
    {
      user,
      showName = false,
      nameClassName,
      showDescription = false,
      description,
      descriptionClassName,
      direction = 'horizontal',
      gap = 'sm',
      size = 'm',
      ...avatarProps
    },
    ref
  ) => {
    const gapClasses = {
      none: '',
      xs: 'gap-1',
      sm: 'gap-2',
      m: 'gap-3',
      l: 'gap-4',
    };

    const directionClasses = {
      horizontal: 'flex-row items-center',
      vertical: 'flex-col items-start',
    };

    return (
      <div 
        ref={ref}
        className={`flex ${directionClasses[direction]} ${gapClasses[gap]}`}
      >
        <Avatar
          src={user.photo}
          alt={user.name}
          name={user.name}
          initials={user.initials}
          size={size}
        />
        
        <div className="flex flex-col">
          {showName && user.name && (
            <span className={`font-medium text-white ${nameClassName || ''}`}>
              {user.name}
            </span>
          )}
          
          {showDescription && (description || user.email) && (
            <span className={`text-sm text-primary-300 ${descriptionClassName || ''}`}>
              {description || user.email}
            </span>
          )}
        </div>
      </div>
    );
  }
);

UserAvatar.displayName = 'UserAvatar';

// Team Avatar
export interface TeamAvatarProps extends Omit<AvatarProps, 'name' | 'initials' | 'alt' | 'src'> {
  team: {
    id?: string;
    name?: string;
    logo?: string;
    abbreviation?: string;
    [key: string]: any;
  };
  size?: AvatarSize;
}

export const TeamAvatar = forwardRef<HTMLImageElement, TeamAvatarProps>(
  (
    {
      team,
      size = 'm',
      ...props
    },
    ref
  ) => {
    const initials = team.abbreviation || (team.name ? getInitials(team.name) : 'T');

    return (
      <Avatar
        ref={ref}
        {...props}
        src={team.logo}
        alt={team.name}
        initials={initials}
        size={size}
        shape="circle"
      />
    );
  }
);

TeamAvatar.displayName = 'TeamAvatar';

// Club Avatar
export interface ClubAvatarProps extends Omit<AvatarProps, 'name' | 'initials' | 'alt' | 'src'> {
  club: {
    id?: string;
    name?: string;
    logo?: string;
    code?: string;
    [key: string]: any;
  };
  size?: AvatarSize;
}

export const ClubAvatar = forwardRef<HTMLImageElement, ClubAvatarProps>(
  (
    {
      club,
      size = 'm',
      ...props
    },
    ref
  ) => {
    const initials = club.code || (club.name ? getInitials(club.name) : 'C');

    return (
      <Avatar
        ref={ref}
        {...props}
        src={club.logo}
        alt={club.name}
        initials={initials}
        size={size}
        shape="circle"
        variant="primary"
      />
    );
  }
);

ClubAvatar.displayName = 'ClubAvatar';

// Exports
export { getInitials };
export default Avatar;
