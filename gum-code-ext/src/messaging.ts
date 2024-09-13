import { KeyedEventListener } from "./KeyedListener";

type MyEvent = { message: string };

export const messagor = new KeyedEventListener<MyEvent>();

setInterval(() => {
    messagor.emit({message: "hi " + new Date().toString()})
}, 1000)
