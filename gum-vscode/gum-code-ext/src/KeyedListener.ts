type Listener<T> = (event: T) => void;

// TODO
// https://chatgpt.com/share/824091ff-e095-4669-ac96-bc50e51bd0f6

export class KeyedEventListener<T> {
  private listeners: Map<string, Listener<T>> = new Map();

  /**
   * Adds a listener with a unique key. If a listener with the same key already exists, it will be replaced.
   * @param key A unique key to identify the listener.
   * @param listener The listener function to be executed when the event is triggered.
   */
  addListener(key: string, listener: Listener<T>): void {
    this.listeners.set(key, listener);
  }

  /**
   * Removes a listener associated with the provided key.
   * @param key The key of the listener to remove.
   */
  removeListener(key: string): void {
    this.listeners.delete(key);
  }

  /**
   * Triggers the event, calling all registered listeners with the provided event data.
   * @param event The event data to pass to each listener.
   */
  emit(event: T): void {
    this.listeners.forEach((listener) => listener(event));
  }
}
