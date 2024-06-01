import {IconButton, InputAdornment, Stack, TextField} from "@mui/material";
import {Clear, ContentCopy} from "@mui/icons-material";
import {Dispatch, ReactNode, useRef} from "react";

export interface AreaProps {
    autoFocus?: boolean;
    label?: ReactNode;
    value: string
    setValue: Dispatch<string>
}

export default function Area({autoFocus, label, value, setValue}: AreaProps) {
    const ref = useRef<HTMLElement>(null)
    return (
        <TextField
            autoFocus={autoFocus}
            fullWidth
            multiline
            minRows={6}
            label={label}
            value={value}
            onChange={event => setValue(event.target.value)}
            InputProps={{
                ref,
                endAdornment: (
                    <InputAdornment position="end">
                        <Stack spacing={3}>
                            <IconButton onClick={() => {
                                const {current} = ref
                                if (!current) return
                                const {firstChild} = current
                                if (!(firstChild instanceof HTMLTextAreaElement)) return
                                firstChild.select()
                                document.execCommand('copy')
                            }}>
                                <ContentCopy/>
                            </IconButton>
                            <IconButton onClick={() => {
                                setValue('')
                                const {current} = ref
                                if (!current) return
                                const {firstChild} = current
                                if (!(firstChild instanceof HTMLTextAreaElement)) return
                                firstChild.focus()
                            }}>
                                <Clear/>
                            </IconButton>
                        </Stack>
                    </InputAdornment>
                )
            }}
        />
    )
}