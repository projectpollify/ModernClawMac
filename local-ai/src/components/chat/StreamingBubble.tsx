import { MessageMetricsRow } from './MessageMetricsRow';
import { MessageContent } from './MessageContent';
import { useSettingsStore } from '@/stores/settingsStore';
import type { MessageMetrics } from '@/types';

interface StreamingBubbleProps {
  content: string;
  metrics?: MessageMetrics | null;
}

export function StreamingBubble({ content, metrics }: StreamingBubbleProps) {
  const { showTokenCount, showResponseMetrics } = useSettingsStore((state) => state.settings);
  const hasVisibleContent = Boolean(content.trim());

  return (
    <div className="flex gap-3">
      <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-secondary text-sm font-medium text-secondary-foreground">
        AI
      </div>

      <div className="max-w-[80%] rounded-2xl rounded-tl-sm bg-secondary px-4 py-3 text-secondary-foreground shadow-sm">
        {hasVisibleContent ? <MessageContent content={content} /> : <p className="text-sm opacity-70">Thinking...</p>}
        {showResponseMetrics ? <MessageMetricsRow metrics={metrics} /> : null}
        {showTokenCount && hasVisibleContent ? (
          <div className="mt-2 text-xs opacity-60">~{Math.max(1, Math.ceil(content.length / 4))} tokens</div>
        ) : null}
        <span className="ml-1 inline-block h-4 w-2 animate-pulse bg-foreground/50 align-middle" />
      </div>
    </div>
  );
}
