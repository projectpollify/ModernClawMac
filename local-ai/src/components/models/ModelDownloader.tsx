import { useState } from 'react';
import { ModelDownloadProgressCard } from '@/components/models/ModelDownloadProgressCard';
import { getModelDisplayName, IS_MAC_MODEL_PROVIDER } from '@/lib/providerConfig';
import { CURATED_FLOOR_MODELS } from '@/lib/voiceCatalog';
import { setupApi } from '@/services/setup';
import { cn } from '@/lib/utils';
import { useAgentStore } from '@/stores/agentStore';
import { useModelStore } from '@/stores/modelStore';
import { useSettingsStore } from '@/stores/settingsStore';

export function ModelDownloader() {
  const [customModel, setCustomModel] = useState('');
  const [switchError, setSwitchError] = useState<string | null>(null);
  const [pendingModelName, setPendingModelName] = useState<string | null>(null);
  const currentModel = useModelStore((state) => state.currentModel);
  const models = useModelStore((state) => state.models);
  const setCurrentModel = useModelStore((state) => state.setCurrentModel);
  const checkStatus = useModelStore((state) => state.checkStatus);
  const downloadModel = useModelStore((state) => state.downloadModel);
  const downloadingModel = useModelStore((state) => state.downloadingModel);
  const downloadProgress = useModelStore((state) => state.downloadProgress);
  const updateActiveAgentDefaultModel = useAgentStore((state) => state.updateActiveAgentDefaultModel);
  const loadSettings = useSettingsStore((state) => state.loadSettings);

  const handleDownload = (name: string) => {
    void downloadModel(name);
  };

  const handleSwitchModel = async (name: string) => {
    if (pendingModelName || name === currentModel) {
      return;
    }

    setSwitchError(null);
    setPendingModelName(name);

    try {
      await setupApi.switchDirectEngineModel(name);
      setCurrentModel(name);
      await updateActiveAgentDefaultModel(name);
      await loadSettings();
      await checkStatus();
    } catch (error) {
      setSwitchError(error instanceof Error ? error.message : String(error));
      await checkStatus();
    } finally {
      setPendingModelName(null);
    }
  };

  if (IS_MAC_MODEL_PROVIDER) {
    const installedNames = new Set(models.map((model) => model.name));

    return (
      <div className="space-y-4">
        <div className="rounded-2xl border border-border bg-background/70 p-4 text-sm text-muted-foreground">
          ModernClawMac can switch between the standard Gemma 4 lanes when those GGUF files are available locally.
          Use the buttons below to move between the conversational E2B lane and the stronger thinking E4B lane.
        </div>

        <div className="flex flex-wrap gap-2">
          {CURATED_FLOOR_MODELS.map((model) => (
            <div key={model.name} className="rounded-xl border border-border bg-background px-3 py-2 text-left text-sm">
              <div className="flex items-center gap-2">
                <div className="font-medium">{getModelDisplayName(model.name)}</div>
                {model.name === currentModel ? (
                  <span className="rounded-full bg-green-500/12 px-2 py-0.5 text-xs text-green-700">Active</span>
                ) : null}
                {installedNames.has(model.name) ? (
                  <span className="rounded-full bg-secondary px-2 py-0.5 text-xs text-secondary-foreground">Loaded</span>
                ) : null}
              </div>
              <div className="mt-1 text-xs text-muted-foreground">{model.description}</div>
              <button
                onClick={() => void handleSwitchModel(model.name)}
                disabled={Boolean(pendingModelName) || model.name === currentModel}
                className={cn(
                  'mt-3 rounded-md px-3 py-1.5 text-sm transition-colors',
                  model.name === currentModel
                    ? 'cursor-default bg-green-500/10 text-green-700'
                    : 'bg-primary text-primary-foreground hover:bg-primary/90 disabled:cursor-not-allowed disabled:opacity-50'
                )}
              >
                {pendingModelName === model.name ? 'Switching...' : model.name === currentModel ? 'Active' : 'Use This Model'}
              </button>
            </div>
          ))}
        </div>

        {switchError ? (
          <div className="rounded-2xl border border-red-500/20 bg-red-500/10 px-4 py-3 text-sm text-red-600">
            {switchError}
          </div>
        ) : null}
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="rounded-2xl border border-border bg-background/70 p-4 text-sm text-muted-foreground">
        ModernClaw is currently tuned around the Gemma 4 family. Start with the primary lane for the strongest local quality, or use the lighter 2B sibling when you want a smaller supported option on the same track.
      </div>

      <div className="flex flex-wrap gap-2">
        {CURATED_FLOOR_MODELS.map((model) => (
          <button
            key={model.name}
            onClick={() => handleDownload(model.name)}
            disabled={Boolean(downloadingModel)}
            className={cn(
              'rounded-xl border border-border bg-background px-3 py-2 text-left text-sm transition-colors',
              'hover:bg-accent hover:text-accent-foreground',
              'disabled:cursor-not-allowed disabled:opacity-50'
            )}
            title={model.description}
          >
            <span className="font-medium">{getModelDisplayName(model.name)}</span>
            <span className="ml-2 text-muted-foreground">{model.size}</span>
          </button>
        ))}
      </div>

      <div className="flex gap-2">
        <input
          type="text"
          value={customModel}
          onChange={(event) => setCustomModel(event.target.value)}
          placeholder="Optional custom model, e.g. llama3.1:8b"
          className="flex-1 rounded-xl border border-border bg-background px-3 py-2 text-sm outline-none transition-colors focus:border-primary/50"
        />
        <button
          onClick={() => handleDownload(customModel)}
          disabled={!customModel.trim() || Boolean(downloadingModel)}
          className="rounded-xl bg-primary px-4 py-2 text-sm text-primary-foreground transition-colors hover:bg-primary/90 disabled:cursor-not-allowed disabled:opacity-50"
        >
          Download
        </button>
      </div>

      {downloadingModel && downloadProgress ? <ModelDownloadProgressCard progress={downloadProgress} /> : null}
    </div>
  );
}
