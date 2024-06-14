export interface ButtonProps {
    className?:     string,
    children?:      JSX.Element | JSX.Element[],
    onClick:        (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => void,
}

export function Button(props: ButtonProps) {
    const classN = (props.className)? ` ${props.className}`:'';
    return (
        <button type="button"
            className={"text-white hover:bg-neutral-700 rounded-full p-1" + classN}
            onClick={(e) => {
                if (props.onClick) {
                    props.onClick(e);
                }
            }}
            >
            {props.children!}
        </button>
    )
}

export function DropDown(props: ButtonProps) {
    const classN = (props.className)? ` ${props.className}`:'';
    return (
        <button type="button"
            className={"text-white hover:bg-neutral-700 rounded-full p-1" + classN}
            onClick={(e) => {
                if (props.onClick) {
                    props.onClick(e);
                }
            }}
            >
            {props.children}
        </button>
    )
}
