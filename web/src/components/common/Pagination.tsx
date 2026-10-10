import React, { forwardRef, HTMLAttributes } from 'react';
import { Icon } from './Icon';
import Button from './Button';
import Select from './Select';
import { cva, type VariantProps } from 'class-variance-authority';

// Pagination container variants
const paginationVariants = cva(
  'flex items-center justify-between flex-wrap gap-4',
  {
    variants: {
      size: {
        sm: 'text-sm',
        m: 'text-base',
        l: 'text-lg',
      },
    },
    defaultVariants: {
      size: 'm',
    },
  }
);

// Pagination size type
type PaginationSize = 'sm' | 'm' | 'l';

// Button size type
const buttonSizes = {
  sm: 'sm' as const,
  m: 'm' as const,
  l: 'l' as const,
};

const selectSizes = {
  sm: 'sm' as const,
  m: 'm' as const,
  l: 'l' as const,
};

// Pagination props
export interface PaginationProps extends HTMLAttributes<HTMLDivElement>, VariantProps<typeof paginationVariants> {
  currentPage: number;
  totalPages: number;
  totalItems: number;
  pageSize: number;
  onPageChange?: (page: number) => void;
  onPageSizeChange?: (size: number) => void;
  pageSizeOptions?: number[];
  showPageSizeSelector?: boolean;
  showTotalItems?: boolean;
  showBoundaryButtons?: boolean;
  showSiblingButtons?: boolean;
  siblingCount?: number;
  boundaryCount?: number;
  disabled?: boolean;
}

// Pagination component
const Pagination = forwardRef<HTMLDivElement, PaginationProps>(
  (
    {
      currentPage = 1,
      totalPages = 1,
      totalItems = 0,
      pageSize = 10,
      onPageChange,
      onPageSizeChange,
      pageSizeOptions = [5, 10, 20, 50, 100],
      showPageSizeSelector = false,
      showTotalItems = true,
      showBoundaryButtons = true,
      showSiblingButtons = true,
      siblingCount = 1,
      size = 'm',
      disabled = false,
      className,
      ...props
    },
    ref
  ) => {
    // Calculate visible pages
    const getVisiblePages = (): (number | string)[] => {
      const visiblePages: (number | string)[] = [];
      
      if (totalPages <= 1) return [1];
      
      // Always show first page
      if (showBoundaryButtons) {
        visiblePages.push(1);
      }
      
      // Calculate range around current page
      const start = Math.max(2, currentPage - siblingCount);
      const end = Math.min(totalPages - 1, currentPage + siblingCount);
      
      // Add ellipsis if needed
      if (showSiblingButtons) {
        if (start > 2) {
          visiblePages.push('...');
        }
        
        // Add pages around current
        for (let i = start; i <= end; i++) {
          if (!visiblePages.includes(i)) {
            visiblePages.push(i);
          }
        }
        
        if (end < totalPages - 1) {
          visiblePages.push('...');
        }
      } else {
        // Simple range without siblings
        if (totalPages <= 5) {
          for (let i = 2; i < totalPages; i++) {
            visiblePages.push(i);
          }
        }
      }
      
      // Always show last page
      if (showBoundaryButtons && totalPages > 1) {
        visiblePages.push(totalPages);
      }
      
      return visiblePages;
    };

    const handlePageChange = (page: number) => {
      if (disabled || page < 1 || page > totalPages) return;
      onPageChange?.(page);
    };

    const handlePrevious = () => {
      handlePageChange(currentPage - 1);
    };

    const handleNext = () => {
      handlePageChange(currentPage + 1);
    };

    const handleFirst = () => {
      handlePageChange(1);
    };

    const handleLast = () => {
      handlePageChange(totalPages);
    };

    const handlePageSizeChange = (value: string) => {
      onPageSizeChange?.(Number(value));
      // Reset to first page when page size changes
      onPageChange?.(1);
    };

    const pageSizeOptionsSelect = pageSizeOptions.map(opt => ({
      value: opt.toString(),
      label: `${opt} / page`,
    }));

    // Get button and select sizes based on pagination size
    const buttonSize = buttonSizes[size as PaginationSize] || 'm';
    const selectSize = selectSizes[size as PaginationSize] || 'm';
    
    // Get icon size based on pagination size
    const iconSize = size === 'sm' ? 'sm' : size === 'l' ? 'l' : 'm';

    // Calculate item range
    const startItem = ((currentPage - 1) * pageSize) + 1;
    const endItem = Math.min(currentPage * pageSize, totalItems);

    return (
      <div 
        ref={ref}
        className={paginationVariants({ size, className })}
        {...props}
      >
        {/* Left side - Page size selector and info */}
        <div className="flex items-center gap-4 flex-wrap">
          {showTotalItems && totalItems > 0 && (
            <span className="text-primary-300">
              {startItem}-{endItem} sur {totalItems}
            </span>
          )}
          
          {showPageSizeSelector && (
            <div className="flex items-center gap-2">
              <span className="text-primary-300">Afficher</span>
              <Select
                size={selectSize}
                variant="outline"
                options={pageSizeOptionsSelect}
                value={pageSize.toString()}
                onChange={handlePageSizeChange}
                disabled={disabled}
              />
            </div>
          )}
        </div>
        
        {/* Right side - Navigation buttons */}
        <div className="flex items-center gap-2">
          {/* First page button */}
          <Button
            size={buttonSize}
            variant="ghost"
            onClick={handleFirst}
            disabled={currentPage === 1 || disabled}
            aria-label="Première page"
          >
            <Icon name="ChevronsLeft" size={iconSize} />
          </Button>
          
          {/* Previous button */}
          <Button
            size={buttonSize}
            variant="ghost"
            onClick={handlePrevious}
            disabled={currentPage === 1 || disabled}
            aria-label="Page précédente"
          >
            <Icon name="ChevronLeft" size={iconSize} />
          </Button>
          
          {/* Page buttons */}
          {getVisiblePages().map((page, index) => (
            typeof page === 'number' ? (
              <Button
                key={page}
                size={buttonSize}
                variant={currentPage === page ? 'primary' : 'ghost'}
                onClick={() => handlePageChange(page)}
                disabled={disabled}
                className={currentPage === page ? 'bg-primary-600 text-white' : ''}
                aria-current={currentPage === page ? 'page' : undefined}
                aria-label={`Aller à la page ${page}`}
              >
                {page}
              </Button>
            ) : (
              <span
                key={`ellipsis-${index}`}
                className="px-3 py-2 text-primary-400 pointer-events-none"
              >
                ...
              </span>
            )
          ))}
          
          {/* Next button */}
          <Button
            size={buttonSize}
            variant="ghost"
            onClick={handleNext}
            disabled={currentPage === totalPages || disabled}
            aria-label="Page suivante"
          >
            <Icon name="ChevronRight" size={iconSize} />
          </Button>
          
          {/* Last page button */}
          <Button
            size={buttonSize}
            variant="ghost"
            onClick={handleLast}
            disabled={currentPage === totalPages || disabled}
            aria-label="Dernière page"
          >
            <Icon name="ChevronsRight" size={iconSize} />
          </Button>
        </div>
      </div>
    );
  }
);

Pagination.displayName = 'Pagination';

// Simple pagination with just previous/next
export interface SimplePaginationProps extends Omit<PaginationProps, 'showPageSizeSelector' | 'showTotalItems' | 'showBoundaryButtons' | 'showSiblingButtons' | 'siblingCount' | 'boundaryCount'> {
  showInfo?: boolean;
}

export const SimplePagination = forwardRef<HTMLDivElement, SimplePaginationProps>(
  (
    {
      currentPage = 1,
      totalPages = 1,
      totalItems = 0,
      pageSize = 10,
      onPageChange,
      size = 'm',
      disabled = false,
      showInfo = true,
      className,
      ...props
    },
    ref
  ) => {
    return (
      <Pagination
        ref={ref}
        currentPage={currentPage}
        totalPages={totalPages}
        totalItems={totalItems}
        pageSize={pageSize}
        onPageChange={onPageChange}
        size={size}
        disabled={disabled}
        showTotalItems={showInfo}
        showBoundaryButtons={false}
        showSiblingButtons={false}
        showPageSizeSelector={false}
        className={className}
        {...props}
      />
    );
  }
);

SimplePagination.displayName = 'SimplePagination';

// Pagination info only
export interface PaginationInfoProps {
  currentPage: number;
  totalPages: number;
  totalItems: number;
  pageSize: number;
  className?: string;
}

export const PaginationInfo: React.FC<PaginationInfoProps> = ({
  currentPage,
  totalItems = 0,
  pageSize = 10,
  className,
}) => {
  const startItem = ((currentPage - 1) * pageSize) + 1;
  const endItem = Math.min(currentPage * pageSize, totalItems);

  return (
    <span className={`text-primary-300 text-sm ${className || ''}`}>
      {startItem}-{endItem} sur {totalItems}
    </span>
  );
};

export default Pagination;
