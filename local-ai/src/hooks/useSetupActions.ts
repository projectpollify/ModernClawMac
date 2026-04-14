import { useState } from 'react';
import { IS_MAC_MODEL_PROVIDER, MODEL_PROVIDER_NAME } from '@/lib/providerConfig';
import { DEFAULT_FLOOR_MODEL, LIGHTWEIGHT_FLOOR_MODEL } from '@/lib/voiceCatalog';
import { setupApi } from '@/services/setup';
import { useMemoryStore } from '@/stores/memoryStore';
import { useModelStore } from '@/stores/modelStore';

function delay(ms: number) {
  return new Promise((resolve) => window.setTimeout(resolve, ms));
}

type ActionNoticeTone = 'success' | 'info';

export function useSetupActions() {
  const checkStatus = useModelStore((state) => state.checkStatus);
  const downloadModel = useModelStore((state) => state.downloadModel);
  const loadModels = useModelStore((state) => state.loadModels);
  const downloadingModel = useModelStore((state) => state.downloadingModel);
  const initializeMemory = useMemoryStore((state) => state.initialize);
  const memoryBasePath = useMemoryStore((state) => state.basePath);

  const [isOpeningDownload, setIsOpeningDownload] = useState(false);
  const [isStartingOllama, setIsStartingOllama] = useState(false);
  const [isInstallingRecommendedModel, setIsInstallingRecommendedModel] = useState(false);
  const [isInitializingWorkspace, setIsInitializingWorkspace] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);
  const [actionNotice, setActionNotice] = useState<{ tone: ActionNoticeTone; message: string } | null>(null);

  const resetFeedback = () => {
    setActionError(null);
    setActionNotice(null);
  };

  const openProviderApp = async () => {
    setIsOpeningDownload(true);
    resetFeedback();

    try {
      await setupApi.openProviderApp();
      setActionNotice({
        tone: 'info',
        message: IS_MAC_MODEL_PROVIDER
          ? `Opened LM Studio. Start the local server on port 1234 there, then load either ${DEFAULT_FLOOR_MODEL} or ${LIGHTWEIGHT_FLOOR_MODEL} before refreshing setup.`
          : 'Opened the Ollama download page. Install it there, then come back here and click Start Ollama.',
      });
    } catch (error) {
      setActionError(String(error));
    } finally {
      setIsOpeningDownload(false);
    }
  };

  const startOllama = async () => {
    setIsStartingOllama(true);
    resetFeedback();

    try {
      await setupApi.startOllama();

      if (IS_MAC_MODEL_PROVIDER) {
        setActionNotice({
          tone: 'info',
          message: `Opened ${MODEL_PROVIDER_NAME}. Start the local server on port 1234 there, then load either ${DEFAULT_FLOOR_MODEL} or ${LIGHTWEIGHT_FLOOR_MODEL} before refreshing setup.`,
        });
      }

      for (let attempt = 0; attempt < 5; attempt += 1) {
        await delay(1200);
        await checkStatus();

        if (useModelStore.getState().ollamaStatus?.running) {
          setActionNotice({
            tone: 'success',
            message: IS_MAC_MODEL_PROVIDER
              ? `${MODEL_PROVIDER_NAME} is responding. The next step is making sure either ${DEFAULT_FLOOR_MODEL} or ${LIGHTWEIGHT_FLOOR_MODEL} is loaded there.`
              : 'Ollama is responding. The next step is installing the recommended model.',
          });
          return true;
        }
      }

      setActionError(
        IS_MAC_MODEL_PROVIDER
          ? `LM Studio is not responding yet. Open LM Studio, start the local server on port 1234, then load either ${DEFAULT_FLOOR_MODEL} or ${LIGHTWEIGHT_FLOOR_MODEL} before refreshing setup.`
          : 'Tried to start Ollama, but it is not responding yet. If this is a fresh install, open Ollama once and then refresh setup.'
      );
      return false;
    } catch (error) {
      setActionError(String(error));
      return false;
    } finally {
      setIsStartingOllama(false);
    }
  };

  const installRecommendedModel = async () => {
    setIsInstallingRecommendedModel(true);
    resetFeedback();

    try {
      if (IS_MAC_MODEL_PROVIDER) {
        await loadModels();
        const { models } = useModelStore.getState();

        if (!models.some((model) => [DEFAULT_FLOOR_MODEL, LIGHTWEIGHT_FLOOR_MODEL].includes(model.name))) {
          setActionError(
            `Load either ${DEFAULT_FLOOR_MODEL} or ${LIGHTWEIGHT_FLOOR_MODEL} inside LM Studio, then refresh setup here.`
          );
          return false;
        }

        setActionNotice({
          tone: 'success',
          message: 'LM Studio already has a standard Gemma 4 model loaded. The next step is making sure the workspace files are ready.',
        });
        return true;
      }

      await downloadModel(DEFAULT_FLOOR_MODEL);
      await loadModels();

      const { error: modelError, models } = useModelStore.getState();
      if (modelError) {
        setActionError(modelError);
        return false;
      }

      const confirmed = models.some((model) => model.name === DEFAULT_FLOOR_MODEL);
      if (!confirmed) {
        setActionError(
          `${DEFAULT_FLOOR_MODEL} has not been confirmed in the installed model list yet. ` +
            'Give Ollama a little more time, then refresh and try again.'
        );
        return false;
      }

      setActionNotice({
        tone: 'success',
        message: `${DEFAULT_FLOOR_MODEL} is installed and verified. The next step is making sure the workspace files are ready.`,
      });
      return true;
    } catch (error) {
      setActionError(String(error));
      return false;
    } finally {
      setIsInstallingRecommendedModel(false);
    }
  };

  const initializeWorkspace = async () => {
    setIsInitializingWorkspace(true);
    resetFeedback();

    try {
      await initializeMemory();

      const memoryError = useMemoryStore.getState().error;
      if (memoryError) {
        setActionError(memoryError);
        return false;
      }

      const nextBasePath = useMemoryStore.getState().basePath ?? memoryBasePath;
      setActionNotice({
        tone: 'success',
        message: nextBasePath
          ? `Workspace files are ready at ${nextBasePath}.`
          : 'Workspace files are ready.',
      });
      return true;
    } catch (error) {
      setActionError(String(error));
      return false;
    } finally {
      setIsInitializingWorkspace(false);
    }
  };

  return {
    openProviderApp,
    startOllama,
    installRecommendedModel,
    initializeWorkspace,
    isOpeningDownload,
    isStartingOllama,
    isInstallingRecommendedModel,
    isInitializingWorkspace,
    isDownloadingAnyModel: Boolean(downloadingModel),
    actionError,
    actionNotice,
    clearActionError: () => setActionError(null),
    clearActionNotice: () => setActionNotice(null),
  };
}
