
export interface ModalProps {
    children: JSX.Element,
    onClick:  (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => void,
}

export function Modal(props: ModalProps) {
    return (
        <div
            className="bg-neutral-700 opacity-85 absolute p-2 inset-x-0 inset-y-0 w-full h-full z-10"
            onClick={(e) => {
                if (props.onClick) {
                    props.onClick(e);
                }
            }}
            >
            {props.children}
        </div>
    )
}
