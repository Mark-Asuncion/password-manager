export interface InputProps {
    getInRef?:               (el: HTMLInputElement) => void,
    value:                string,
    placeholder:          string,
    // nameId:               string,
    classContainer?:      string,
    onChange:             (v: string) => void,
    updator:              () => void,
}

export function MInput(props: InputProps) {
    return (
        <div className={( props.classContainer )? props.classContainer:""}>
            <div className="relative rounded-md shadow-sm">
                <input
                    ref={(el) => {
                        if (props.getInRef && el) {
                            props.getInRef(el);
                        }
                    }}
                    type="text"
                    // name={props.nameId}
                    // id={props.nameId}
                    className="block bg-neutral-900 w-full rounded-md border-0 py-2 pl-3
                    ring-1 ring-inset ring-neutral-700 placeholder:text-gray-400
                    focus:ring-2 focus:ring-inset focus:ring-blue-500 focus:outline-none sm:text-sm sm:leading-6"
                    placeholder={props.placeholder}
                    onChange={ (e) => {
                        props.onChange(e.target.value);
                    }}
                    value={props.value}
                    onBlur={(_) => {
                        props.updator();
                    }}
                />
            </div>
        </div>
    )
}
