import { useEffect, useRef } from "react";

export interface ErrorProps {
    title:   string,
    content: string,
    onClose: () => void
};

export function Error(props: ErrorProps) {
    return (
        <div
            className="rounded-md bg-neutral-900 shadow-black shadow-lg fixed left-1/2 top-1/2 text-white z-20
            translate-x-[-50%] translate-y-[-50%] min-w-52 min-h-52 flex flex-col">
            <h1 className="text-xl font-bold border-neutral-700 border-b text-red-500 uppercase p-4 text-left">{props.title}</h1>
            <h2 className="text-md p-4 grow">
                {props.content}
            </h2>
            <button type="button" className="border-t border-neutral-700 hover:brightness-50 p-2 text-center w-full"
                onClick={(_) => {
                    props.onClose();
                }}
            >Close</button>
        </div>
    )
}

export interface NotificationProps {
    children: JSX.Element | JSX.Element[],
};

export function Notifications(props: NotificationProps) {
    return (
        <div className="z-20 flex flex-col-reverse fixed right-0 bottom-0 mr-4 mb-4 gap-2">
            { props.children }
        </div>
    )
}

export interface NotifyProps {
    content: string,
    onClose: () => void
};

export function Notify(props: NotifyProps) {
    const divRef = useRef<HTMLDivElement | null>(null)
    useEffect(() => {
        let timer = null;
        if (divRef) {
            timer = setTimeout(props.onClose, 10000);
        }
        return () => {
            if (timer)
                clearTimeout(timer);
        }
    }, [divRef]);

    return (
        <div ref={divRef} className="ml-auto w-max text-white p-2 rounded-md bg-neutral-900 shadow-black shadow-lg flex flex-row items-center">
            <p className="p-2 grow">{props.content}</p>
            <button type="button" className="hover:brightness-50 p-2 text-center w-max"
                onClick={(_) => {
                    props.onClose();
                }}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor"
                    className="size-6">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M6 18 18 6M6 6l12 12" />
                </svg>
            </button>
        </div>
    )
}
