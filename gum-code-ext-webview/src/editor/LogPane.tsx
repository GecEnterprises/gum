import { create } from 'zustand';
import { CSSProperties } from 'preact/compat';

type LogStore = {
    entry: string[]
    entryLimit: number
    append: (str: string) => void
}

export const useLoggerStore = create<LogStore>((set) => ({
    entry: [],
    entryLimit: 30,
    append: (str) =>
        set((state) => {
            const updatedEntry = [...state.entry, str];
            if (updatedEntry.length > state.entryLimit) {
                updatedEntry.shift();
            }
            return { entry: updatedEntry };
        }),
}));

type LogPaneProps = {
    style?: CSSProperties; // Optional style prop
};

export const LogPane: React.FC<LogPaneProps> = ({ style }) => {
    const logger = useLoggerStore();

    return (
      <div style={style}>
        {logger.entry.map((message, index) => (
          <div key={index}>{message}</div>
        ))}
      </div>
    );
  };