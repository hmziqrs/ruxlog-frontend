import { useEffect, useMemo, useRef, useState } from 'react';

export function useDidMount() {
  const mounted = useRef<boolean  | undefined>(undefined);
  useEffect(() => {
    if (!mounted.current) {
      // do componentDidMount logic
      mounted.current = true;
    }
  }, []);
  return !!mounted.current;
}

export function usePrev<K>(value: K): K | null {
  const ref = useRef<K | undefined>(undefined);
  useEffect(() => {
    ref.current = value;
  }, [value]); // Only re-run if value changes
  return ref.current as K;
}

export function useSearch(initial = '') {
  const [query, setQuery] = useState(initial);

  return useMemo(
    () => ({
      query,
      setQuery
    }),
    [query]
  );
}

export function useTabs(initial = 0) {
  const [tab, setTab] = useState(initial);

  return useMemo(
    () => ({
      tab,
      setTab
    }),
    [tab]
  );
}

export function useBoolEngine(init = false) {
  const [bool, setBool] = useState<boolean>(init);

  const set = (flag: boolean) => setBool(flag);
  const toggle = () => setBool((s) => !s);
  const setTrue = () => setBool(true);
  const setFalse = () => setBool(false);

  return { bool, toggle, setTrue, setFalse, set };
}

export function useLoader(initialLoading = false) {
  const [loading, setLoading] = useState<boolean>(initialLoading);

  return {
    loading,
    start: () => setLoading(true),
    stop: () => setLoading(false)
  };
}

export function useDebounce<T>(value: T, delay: number): T {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);

  useEffect(() => {
    // Update debounced value after delay
    const handler = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    // Cancel the timeout if value changes (also on delay change or unmount)
    // This is how we prevent debounced value from updating if value is changed ...
    // .. within the delay period. Timeout gets cleared and restarted.
    return () => {
      clearTimeout(handler);
    };
  }, [value, delay]);

  return debouncedValue;
}
