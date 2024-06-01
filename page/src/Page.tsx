import {Container, Grid, Typography} from "@mui/material";
import Area from "./Area.tsx";
import {useState} from "react";
import {decode, encode} from "ns9_1";

export default function Page() {
    const [a, setA] = useState('')
    const [b, setB] = useState('')
    return (
        <Container>
            <Typography variant="h2" gutterBottom>喵喵隐者 9.1</Typography>
            <Grid container spacing={2}>
                <Grid item xs={12} md={6}>
                    <Area
                        autoFocus
                        label="原文"
                        value={a}
                        setValue={value => {
                            setA(value)
                            setB(encode(value))
                        }}
                    />
                </Grid>
                <Grid item xs={12} md={6}>
                    <Area
                        label="乱码"
                        value={b}
                        setValue={value => {
                            setB(value)
                            setA(decode(value))
                        }}
                    />
                </Grid>
            </Grid>
        </Container>
    )
}