import { update } from "../../utils/invoker"

export interface InputProps {
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
            <div className="relative mt-2 rounded-md shadow-sm">
                <input
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
                    onBlur={(e) => {
                        props.updator();
                    }}
                />
            </div>
        </div>
    )
}
