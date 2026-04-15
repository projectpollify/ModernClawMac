import { cn } from '@/lib/utils';
import type { ModelDownloadProgress } from '@/stores/modelStore';

export function ModelDownloadProgressCard({
  progress,
  className,
}: {
  progress: ModelDownloadProgress;
  className?: string;
}) {
  const progressLabel =
    typeof progress.completed === 'number' && typeof progress.total === 'number'
      ? `${formatBytes(progress.completed)} of ${formatBytes(progress.total)}`
      : null;

  return (
    <div className={cn('rounded-2xl border border-primary/20 bg-primary/5 p-4', className)}>
      <div className="flex items-start justify-between gap-3">
        <div>
          <p className="text-sm font-medium">Downloading {progress.model}</p>
          <p className="mt-1 text-sm text-muted-foreground">{humanizeStatus(progress.status)}</p>
        </div>
        <div className="text-right">
          <p className="text-sm font-medium">{typeof progress.percent === 'number' ? `${progress.percent}%` : 'Working...'}</p>
          {progressLabel ? <p className="mt-1 text-xs text-muted-foreground">{progressLabel}</p> : null}
        </div>
      </div>

      <div className="mt-3 h-2 overflow-hidden rounded-full bg-primary/10">
        <div
          className={cn(
            'h-full rounded-full bg-primary transition-[width]',
            typeof progress.percent === 'number' ? 'duration-300 ease-out' : 'animate-pulse w-1/3'
          )}
          style={typeof progress.percent === 'number' ? { width: `${progress.percent}%` } : undefined}
        />
      </div>

      <p className="mt-3 text-xs text-muted-foreground">
        Large model downloads can take a while on older machines or slower connections.
      </p>
    </div>
  );
}

function humanizeStatus(status: string) {
  if (!status) {
    return 'Working...';
  }

  return status.charAt(0).toUpperCase() + status.slice(1);
}

function formatBytes(bytes: number) {
  if (bytes < 1024) {
    return `${bytes} B`;
  }

  const units = ['KB', 'MB', 'GB', 'TB'];
  let value = bytes;
  let unitIndex = -1;

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex += 1;
  }

  return `${value.toFixed(value >= 10 ? 0 : 1)} ${units[unitIndex]}`;
}
