import { forwardRef, HTMLAttributes, ReactNode } from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { Icon, IconName } from './Icon';

// Badge variants
const badgeVariants = cva(
  'inline-flex items-center font-medium rounded-full px-2.5 py-0.5 text-xs',
  {
    variants: {
      variant: {
        default: 'bg-neutral-100 text-neutral-800',
        primary: 'bg-primary-100 text-primary-800',
        secondary: 'bg-secondary-100 text-secondary-800',
        success: 'bg-success-100 text-success-800',
        warning: 'bg-warning-100 text-warning-800',
        danger: 'bg-danger-100 text-danger-800',
        info: 'bg-info-100 text-info-800',
        gold: 'bg-primary-500 text-white',
        subtle: 'bg-neutral-200 text-neutral-700',
      },
      size: {
        xs: 'px-2 py-0.5 text-xs',
        sm: 'px-2.5 py-0.5 text-xs',
        m: 'px-3 py-1 text-sm',
        l: 'px-4 py-1.5 text-sm',
      },
      dot: {
        true: 'pr-1',
        false: '',
      },
    },
    compoundVariants: [
      {
        variant: 'gold',
        className: 'shadow-gold-sm',
      },
    ],
    defaultVariants: {
      variant: 'default',
      size: 'sm',
      dot: false,
    },
  }
);

// Badge props
export interface BadgeProps extends 
  HTMLAttributes<HTMLSpanElement>,
  VariantProps<typeof badgeVariants> {
  children?: ReactNode;
  icon?: IconName;
  dotColor?: string;
  showDot?: boolean;
}

// Badge component
const Badge = forwardRef<HTMLSpanElement, BadgeProps>(
  (
    {
      className,
      variant,
      size,
      dot,
      children,
      icon,
      dotColor = 'currentColor',
      showDot = false,
      ...props
    },
    ref
  ) => {
    const baseStyles = badgeVariants({ variant, size, dot });

    return (
      <span
        ref={ref}
        className={`${baseStyles} ${className || ''}`}
        {...props}
      >
        {showDot && (
          <span 
            style={{
              width: '6px',
              height: '6px',
              backgroundColor: dotColor,
              borderRadius: '50%',
              marginRight: '4px',
              flexShrink: 0,
            }}
          />
        )}
        {icon && <Icon name={icon} size="xs" />}
        {children && <span>{children}</span>}
      </span>
    );
  }
);

Badge.displayName = 'Badge';

// Status Badge for match/competition status
interface StatusBadgeProps extends Omit<BadgeProps, 'variant' | 'children'> {
  status: 'scheduled' | 'in_progress' | 'completed' | 'postponed' | 'cancelled' | 'disputed' | 'registered' | 'pending' | 'active' | 'inactive' | 'verified' | 'unverified';
}

export const StatusBadge = forwardRef<HTMLSpanElement, StatusBadgeProps>(
  ({ status, ...props }, ref) => {
    const getStatusVariant = (): BadgeProps['variant'] => {
      switch (status) {
        case 'in_progress':
        case 'active':
          return 'gold';
        case 'completed':
        case 'verified':
          return 'success';
        case 'scheduled':
        case 'pending':
          return 'info';
        case 'postponed':
        case 'disputed':
          return 'warning';
        case 'cancelled':
        case 'inactive':
        case 'unverified':
          return 'danger';
        case 'registered':
          return 'primary';
        default:
          return 'default';
      }
    };

    const getStatusText = (): string => {
      switch (status) {
        case 'scheduled': return 'À venir';
        case 'in_progress': return 'EN DIRECT';
        case 'completed': return 'Terminé';
        case 'postponed': return 'Reporté';
        case 'cancelled': return 'Annulé';
        case 'disputed': return 'Contesté';
        case 'registered': return 'Inscrit';
        case 'pending': return 'En attente';
        case 'active': return 'Actif';
        case 'inactive': return 'Inactif';
        case 'verified': return 'Vérifié';
        case 'unverified': return 'Non vérifié';
        default: return status;
      }
    };

    return (
      <Badge 
        ref={ref} 
        variant={getStatusVariant()}
        showDot={status === 'in_progress' || status === 'active'}
        dotColor={status === 'in_progress' || status === 'active' ? 'var(--color-primary-500)' : undefined}
        {...props}
      >
        {getStatusText()}
      </Badge>
    );
  }
);

StatusBadge.displayName = 'StatusBadge';

// Category Badge for competitions
interface CategoryBadgeProps extends Omit<BadgeProps, 'variant' | 'children'> {
  category: string;
}

export const CategoryBadge = forwardRef<HTMLSpanElement, CategoryBadgeProps>(
  ({ category, ...props }, ref) => {
    const getCategoryText = (): string => {
      switch (category) {
        case 'individual': return 'Individuel';
        case 'doublette': return 'Doublette';
        case 'triplette': return 'Triplette';
        default: return category;
      }
    };

    return (
      <Badge ref={ref} variant="primary" {...props}>
        {getCategoryText()}
      </Badge>
    );
  }
);

CategoryBadge.displayName = 'CategoryBadge';

// Material Badge for competition material
interface MaterialBadgeProps extends Omit<BadgeProps, 'variant' | 'children'> {
  material: string;
}

export const MaterialBadge = forwardRef<HTMLSpanElement, MaterialBadgeProps>(
  ({ material, ...props }, ref) => {
    const getMaterialText = (): string => {
      switch (material) {
        case 'fonte': return 'Fonte';
        case 'laiton': return 'Laiton';
        case 'bois': return 'Bois';
        case 'mixte': return 'Mixte';
        default: return material;
      }
    };

    return (
      <Badge ref={ref} variant="secondary" {...props}>
        {getMaterialText()}
      </Badge>
    );
  }
);

MaterialBadge.displayName = 'MaterialBadge';

// Role Badge for users
export const RoleBadge = forwardRef<HTMLSpanElement, { role: string } & Omit<BadgeProps, 'children'>>(
  ({ role, ...props }, ref) => {
    const getRoleText = (): string => {
      switch (role) {
        case 'super_admin': return 'Super Admin';
        case 'commission_admin': return 'Admin Commission';
        case 'club_admin': return 'Admin Club';
        case 'arbitre': return 'Arbitre';
        case 'player': return 'Joueur';
        default: return role;
      }
    };

    return (
      <Badge ref={ref} variant={role === 'super_admin' ? 'danger' : 'primary'} {...props}>
        {getRoleText()}
      </Badge>
    );
  }
);

RoleBadge.displayName = 'RoleBadge';

// Position Badge for teams in standings
export const PositionBadge = forwardRef<HTMLSpanElement, { position: number } & Omit<BadgeProps, 'variant'>>(
  ({ position, ...props }, ref) => {
    return (
      <Badge 
        ref={ref} 
        variant={position === 1 ? 'gold' : position <= 3 ? 'success' : 'default'}
        {...props}
      >
        {position}
      </Badge>
    );
  }
);

PositionBadge.displayName = 'PositionBadge';

// Form Badge for team form (win/loss streak)
export const FormBadge = forwardRef<HTMLDivElement, { form: ('win' | 'draw' | 'loss')[] }>(
  ({ form }, ref) => {
    return (
      <div ref={ref} style={styles.formBadgeContainer}>
        {form.slice(0, 5).map((result, index) => (
          <Badge 
            key={index} 
            variant={result === 'win' ? 'success' : result === 'draw' ? 'warning' : 'danger'}
            size="xs"
            style={styles.formBadgeItem}
          >
            {result === 'win' && 'V'}
            {result === 'draw' && 'N'}
            {result === 'loss' && 'D'}
          </Badge>
        ))}
      </div>
    );
  }
);

FormBadge.displayName = 'FormBadge';

// Styles
const styles = {
  formBadgeContainer: {
    display: 'flex',
    gap: '2px',
    flexWrap: 'nowrap' as const,
  } as React.CSSProperties,
  
  formBadgeItem: {
    padding: '1px 4px',
    fontSize: '10px',
    fontWeight: '600',
  } as React.CSSProperties,
};

// Named exports
export { Badge };
export default Badge;