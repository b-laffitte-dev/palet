import React, { forwardRef, HTMLAttributes, TdHTMLAttributes, ThHTMLAttributes } from 'react';
import { Icon, IconName } from './Icon';
import { StatusBadge } from './Badge';
import { cva, type VariantProps } from 'class-variance-authority';

// Table container variants
const tableContainerVariants = cva(
  'w-full overflow-x-auto rounded-lg border border-primary-700',
  {
    variants: {
      variant: {
        default: '',
        striped: '',
        hoverable: '',
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

// Table variants
const tableVariants = cva(
  'w-full table-fixed',
  {
    variants: {
      variant: {
        default: '',
        striped: '',
        hoverable: '',
      },
      size: {
        xs: 'text-xs',
        sm: 'text-sm',
        m: 'text-base',
        l: 'text-lg',
      },
    },
  }
);

// Table cell variants
const tableCellVariants = cva(
  'p-4 text-left align-middle',
  {
    variants: {
      variant: {
        header: 'bg-primary-800 text-primary-200 font-semibold uppercase tracking-wider text-xs',
        body: 'bg-primary-900 text-white border-t border-primary-700/50',
        footer: 'bg-primary-800 text-primary-300 font-medium',
      },
      size: {
        xs: 'px-2 py-1',
        sm: 'px-3 py-2',
        m: 'px-4 py-3',
        l: 'px-6 py-4',
      },
    },
    defaultVariants: {
      variant: 'body',
      size: 'm',
    },
  }
);

// Table props
export interface TableProps extends HTMLAttributes<HTMLTableElement>, VariantProps<typeof tableContainerVariants> {
  children: React.ReactNode;
  striped?: boolean;
  hoverable?: boolean;
  compact?: boolean;
  className?: string;
}

// Table component
const Table = forwardRef<HTMLTableElement, TableProps>(
  (
    {
      children,
      variant = 'default',
      size = 'm',
      striped = false,
      hoverable = false,
      className,
      ...props
    },
    ref
  ) => {
    return (
      <div className={tableContainerVariants({ variant, size, className })}>
        <table 
          ref={ref}
          className={tableVariants({ variant, size, className: striped ? 'striped' : hoverable ? 'hoverable' : '' })}
          {...props}
        >
          {children}
        </table>
      </div>
    );
  }
);

Table.displayName = 'Table';

// TableHeader props
export interface TableHeaderProps extends HTMLAttributes<HTMLTableSectionElement> {
  children: React.ReactNode;
}

// TableHeader component
const TableHeader = forwardRef<HTMLTableSectionElement, TableHeaderProps>(
  ({ children, ...props }, ref) => (
    <thead ref={ref} {...props}>
      {children}
    </thead>
  )
);

TableHeader.displayName = 'TableHeader';

// TableBody props
export interface TableBodyProps extends HTMLAttributes<HTMLTableSectionElement> {
  children: React.ReactNode;
}

// TableBody component
const TableBody = forwardRef<HTMLTableSectionElement, TableBodyProps>(
  ({ children, ...props }, ref) => (
    <tbody ref={ref} {...props}>
      {children}
    </tbody>
  )
);

TableBody.displayName = 'TableBody';

// TableFooter props
export interface TableFooterProps extends HTMLAttributes<HTMLTableSectionElement> {
  children: React.ReactNode;
}

// TableFooter component
const TableFooter = forwardRef<HTMLTableSectionElement, TableFooterProps>(
  ({ children, ...props }, ref) => (
    <tfoot ref={ref} {...props}>
      {children}
    </tfoot>
  )
);

TableFooter.displayName = 'TableFooter';

// TableRow props
export interface TableRowProps extends HTMLAttributes<HTMLTableRowElement> {
  children: React.ReactNode;
  selected?: boolean;
  onClick?: () => void;
  hoverable?: boolean;
}

// TableRow component
const TableRow = forwardRef<HTMLTableRowElement, TableRowProps>(
  (
    {
      children,
      selected = false,
      onClick,
      hoverable = false,
      className,
      ...props
    },
    ref
  ) => {
    return (
      <tr
        ref={ref}
        className={`{
          ${selected ? 'bg-primary-700/50' : ''}
          ${hoverable ? 'hover:bg-primary-700/30 transition-colors cursor-pointer' : ''}
          ${onClick ? 'cursor-pointer' : ''}
          ${className || ''}
        }`}
        onClick={onClick}
        {...props}
      >
        {children}
      </tr>
    );
  }
);

TableRow.displayName = 'TableRow';

// TableHeadCell props
export interface TableHeadCellProps extends ThHTMLAttributes<HTMLTableCellElement>, VariantProps<typeof tableCellVariants> {
  children: React.ReactNode;
  sortable?: boolean;
  sortDirection?: 'asc' | 'desc' | null;
  onSort?: () => void;
  sortIcon?: IconName;
}

// TableHeadCell component
const TableHeadCell = forwardRef<HTMLTableCellElement, TableHeadCellProps>(
  (
    {
      children,
      variant = 'header',
      size = 'm',
      sortable = false,
      sortDirection = null,
      onSort,
      sortIcon = 'ArrowUpDown',
      className,
      ...props
    },
    ref
  ) => {
    const getSortIcon = (): IconName => {
      if (sortDirection === 'asc') return 'ChevronUp';
      if (sortDirection === 'desc') return 'ChevronDown';
      return sortIcon;
    };

    return (
      <th
        ref={ref}
        className={tableCellVariants({ variant, size, className })}
        onClick={sortable ? onSort : undefined}
        {...props}
      >
        <div className="flex items-center justify-between gap-2">
          <span>{children}</span>
          {sortable && (
            <Icon 
              name={getSortIcon()}
              size={size === 'xs' ? 'xs' : size === 'l' ? 'l' : 'm'}
              className={`text-primary-400 ${sortDirection ? 'text-primary-200' : ''}`}
            />
          )}
        </div>
      </th>
    );
  }
);

TableHeadCell.displayName = 'TableHeadCell';

// TableCell props
export interface TableCellProps extends TdHTMLAttributes<HTMLTableCellElement>, VariantProps<typeof tableCellVariants> {
  children: React.ReactNode;
  status?: string;
  badge?: string | React.ReactNode;
}

// TableCell component
const TableCell = forwardRef<HTMLTableCellElement, TableCellProps>(
  (
    {
      children,
      variant = 'body',
      size = 'm',
      status,
      badge,
      className,
      ...props
    },
    ref
  ) => {
    return (
      <td
        ref={ref}
        className={tableCellVariants({ variant, size, className })}
        {...props}
      >
        {status && !badge ? (
          <StatusBadge status={status as any} />
        ) : badge ? (
          <span className="inline-flex">{badge}</span>
        ) : (
          children
        )}
      </td>
    );
  }
);

TableCell.displayName = 'TableCell';

// TableCell with actions
interface TableCellWithActionsProps extends TableCellProps {
  actions?: React.ReactNode[];
}

const TableCellWithActions = forwardRef<HTMLTableCellElement, TableCellWithActionsProps>(
  (
    {
      children,
      actions = [],
      variant = 'body',
      size = 'm',
      className,
      ...props
    },
    ref
  ) => {
    return (
      <td
        ref={ref}
        className={tableCellVariants({ variant, size, className: `whitespace-nowrap ${className || ''}` })}
        {...props}
      >
        <div className="flex items-center gap-4">
          <span className="truncate">{children}</span>
          {actions.length > 0 && (
            <div className="flex items-center gap-2">
              {actions.map((action, index) => (
                <span key={index}>{action}</span>
              ))}
            </div>
          )}
        </div>
      </td>
    );
  }
);

TableCellWithActions.displayName = 'TableCellWithActions';

// Empty state component for tables
export interface TableEmptyProps {
  message?: string;
  description?: string;
  icon?: IconName;
  action?: React.ReactNode;
  colSpan?: number;
  className?: string;
}

export const TableEmpty: React.FC<TableEmptyProps> = ({
  message = 'Aucune donnée disponible',
  description,
  icon = 'Database',
  action,
  colSpan = 1,
  className,
}) => {
  return (
    <TableRow>
      <TableCell colSpan={colSpan} className={`text-center py-12 ${className || ''}`}>
        <div className="flex flex-col items-center justify-center gap-4">
          <Icon name={icon} size="xl" className="text-primary-500" />
          <div className="flex flex-col items-center">
            <h3 className="text-h5 font-semibold text-white">{message}</h3>
            {description && (
              <p className="text-primary-300 text-body-s mt-1">{description}</p>
            )}
          </div>
          {action && <div className="mt-4">{action}</div>}
        </div>
      </TableCell>
    </TableRow>
  );
};

// Table with sorting and filtering
export interface SortableTableProps<T> extends Omit<TableProps, 'children'> {
  data: T[];
  columns: {
    key: keyof T | string;
    label: string;
    sortable?: boolean;
    render?: (item: T) => React.ReactNode;
    className?: string;
  }[];
  keyExtractor?: (item: T) => string;
  sortBy?: keyof T | string;
  sortDirection?: 'asc' | 'desc' | null;
  onSort?: (key: keyof T | string) => void;
  rowClassName?: string | ((item: T) => string);
  emptyMessage?: string;
  loading?: boolean;
  loadingRows?: number;
}

function SortableTable<T extends Record<string, any>>(
  props: SortableTableProps<T>
): React.ReactElement {
  const {
    data,
    columns,
    keyExtractor = (item: T) => item.id || Math.random().toString(),
    sortBy,
    sortDirection,
    onSort,
    rowClassName,
    emptyMessage,
    loading = false,
    loadingRows = 5,
    ...tableProps
  } = props;

  const handleSort = (key: keyof T | string) => {
    const column = columns.find(c => c.key === key);
    if (column?.sortable && onSort) {
      onSort(key);
    }
  };

  const getSortDirection = (key: keyof T | string): 'asc' | 'desc' | null => {
    if (sortBy === key) return sortDirection || null;
    return null;
  };

  if (loading) {
    return (
      <Table {...tableProps}>
        <TableHeader>
          <TableRow>
            {columns.map(column => (
              <TableHeadCell
                key={String(column.key)}
                sortable={column.sortable}
                sortDirection={getSortDirection(column.key)}
                onSort={() => handleSort(column.key)}
              >
                {column.label}
              </TableHeadCell>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {Array.from({ length: loadingRows }).map((_, index) => (
            <TableRow key={index} className="animate-pulse">
              {columns.map(column => (
                <TableCell key={String(column.key)}>
                  <div className="h-4 bg-primary-700 rounded w-3/4" />
                </TableCell>
              ))}
            </TableRow>
          ))}
        </TableBody>
      </Table>
    );
  }

  if (data.length === 0) {
    return (
      <Table {...tableProps}>
        <TableHeader>
          <TableRow>
            {columns.map(column => (
              <TableHeadCell
                key={String(column.key)}
                sortable={column.sortable}
                sortDirection={getSortDirection(column.key)}
                onSort={() => handleSort(column.key)}
              >
                {column.label}
              </TableHeadCell>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableEmpty 
            message={emptyMessage || 'Aucune donnée disponible'}
            colSpan={columns.length}
          />
        </TableBody>
      </Table>
    );
  }

  return (
    <Table {...tableProps}>
      <TableHeader>
        <TableRow>
          {columns.map(column => (
            <TableHeadCell
              key={String(column.key)}
              sortable={column.sortable}
              sortDirection={getSortDirection(column.key)}
              onSort={() => handleSort(column.key)}
            >
              {column.label}
            </TableHeadCell>
          ))}
        </TableRow>
      </TableHeader>
      <TableBody>
        {data.map(item => (
          <TableRow
            key={keyExtractor(item)}
            className={typeof rowClassName === 'function' ? rowClassName(item) : rowClassName}
          >
            {columns.map(column => (
              <TableCell key={String(column.key)}>
                {column.render ? column.render(item) : String(item[column.key as keyof T])}
              </TableCell>
            ))}
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

// Table with row selection
export interface SelectableTableProps<T> extends Omit<SortableTableProps<T>, 'rowClassName' | 'onSort' | 'onSelect'> {
  selectedIds?: string[];
  onSelect?: (id: string) => void;
  onSelectAll?: () => void;
  selectable?: boolean;
  selectAllDisabled?: boolean;
}

function SelectableTable<T extends Record<string, any>>(
  props: SelectableTableProps<T>
): React.ReactElement {
  const {
    data,
    columns,
    selectedIds = [],
    onSelect,
    onSelectAll,
    selectable = false,
    selectAllDisabled = false,
    ...restProps
  } = props;

  const allSelected = data.length > 0 && selectedIds.length === data.length;
  const someSelected = selectedIds.length > 0 && selectedIds.length < data.length;
  const indeterminate = someSelected && !allSelected;

  const handleSelectAll = () => {
    if (onSelectAll) {
      onSelectAll();
    }
  };

  const isSelected = (id: string) => selectedIds.includes(id);

  const tableColumns = selectable ? [
    {
      key: 'select',
      label: '',
      sortable: false,
      className: 'w-12',
      render: () => (
        <input
          type="checkbox"
          checked={allSelected}
          ref={el => {
            if (el) el.indeterminate = indeterminate;
          }}
          onChange={handleSelectAll}
          disabled={selectAllDisabled}
          className="w-4 h-4 rounded border-primary-600 bg-transparent focus:ring-2 focus:ring-gold-500"
          aria-label="Sélectionner tout"
        />
      ),
    },
    ...columns,
  ] : columns;

  return (
    <SortableTable
      {...restProps}
      data={data}
      columns={tableColumns}
      keyExtractor={props.keyExtractor}
      rowClassName={(item) => {
        const id = props.keyExtractor ? props.keyExtractor(item) : (item as any).id;
        return isSelected(id) ? 'bg-primary-700/30' : '';
      }}
    />
  );
}

// Exports
export {
  Table,
  TableHeader,
  TableBody,
  TableFooter,
  TableRow,
  TableHeadCell,
  TableCell,
  TableCellWithActions,
  SortableTable,
  SelectableTable,
};

export default Table;
