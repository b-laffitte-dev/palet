export interface LiveIndicatorProps {
  count?: number;
  animated?: boolean;
}

export function LiveIndicator({ count = 0 }: LiveIndicatorProps) {
  return (
    <div style={styles.container}>
      <span style={styles.dot} />
      <span style={styles.text}>EN DIRECT</span>
      {count > 0 && (
        <span style={styles.count}>{count}</span>
      )}
    </div>
  );
}

const styles = {
  container: {
    display: 'flex',
    alignItems: 'center',
    gap: 'var(--spacing-2)',
    padding: 'var(--spacing-1) var(--spacing-3)',
    backgroundColor: 'rgba(217, 119, 6, 0.15)',
    borderRadius: 'var(--radius-full)',
    fontSize: 'var(--text-body-xs)',
    fontWeight: 'var(--font-weight-semibold)',
    color: 'var(--color-primary-500)',
    animation: 'pulse 2s ease-in-out infinite',
  } as React.CSSProperties,
  
  dot: {
    width: '8px',
    height: '8px',
    backgroundColor: 'var(--color-primary-500)',
    borderRadius: 'var(--radius-full)',
    boxShadow: '0 0 6px var(--color-primary-500)',
  } as React.CSSProperties,
  
  text: {
    letterSpacing: 'var(--letter-spacing-wide)',
  } as React.CSSProperties,
  
  count: {
    backgroundColor: 'var(--color-primary-600)',
    color: 'white',
    padding: 'var(--spacing-0.5) var(--spacing-2)',
    borderRadius: 'var(--radius-full)',
    fontSize: 'var(--text-body-xs)',
    fontWeight: 'var(--font-weight-bold)',
  } as React.CSSProperties,
};

export default LiveIndicator;