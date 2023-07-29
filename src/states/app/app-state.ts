import { invoke, message } from '../../wrappers/invoke-wrapper';
import { ref, computed } from 'vue';

const sendRequest = () => {
  return invoke(message.GENERATE_EXCEL_FILE)
}

type RequestResult = Awaited<ReturnType<typeof sendRequest>>;

export const setup = () => {
  const _isLoaded = ref(false);
  const _isLoading = ref(false);
  const _hasError = ref(false);
  const _requestResult = ref<RequestResult | null>(null);
  const _errorResult = ref<unknown | null>(null);

  
  const isLoaded = computed(() => _isLoaded.value);
  const isLoading = computed(() => _isLoading.value);
  const hasError = computed(() => _hasError.value);
  const result = computed(() => _requestResult.value);
  const error = computed(() => _errorResult.value);
  const hasSuccess = computed(() => _isLoaded.value && !_hasError.value);
  const notLoadedOrHasErrors = computed(() => !_isLoaded.value || _hasError.value);

  const markError = (err: unknown) => {
    console.log({ err });
    _hasError.value = true;
    _errorResult.value = err;
    _isLoaded.value = true;
    _isLoading.value = false;
  };

  const markSuccess = (result: RequestResult) => {
    console.log({ result });
    _hasError.value = false;
    _errorResult.value = null;
    _isLoaded.value = true;
    _isLoading.value = false;
    _requestResult.value = result;
  };

  const markDefault = () => {
    _isLoaded.value = false;
    _isLoading.value = false;
    _hasError.value = false;
    _requestResult.value = null;
    _errorResult.value = null;
  };

  const markPending = () => {
    _isLoading.value = true;
  };

  const generateFile = () => {
    markDefault();
    markPending();

    sendRequest()
      .then(markSuccess)
      .catch(markError)
  };

  return {
    generateFile,
    isLoaded,
    isLoading,
    hasError,
    hasSuccess,
    result,
    error,
    notLoadedOrHasErrors,
  };
}
