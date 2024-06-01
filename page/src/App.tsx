import useMediaQuery from '@mui/material/useMediaQuery';
import {createTheme, ThemeProvider} from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import Page from "./Page.tsx";
import {useMemo} from "react";

export default function App() {
    const prefersDarkMode = useMediaQuery('(prefers-color-scheme: dark)');

    const theme = useMemo(
        () =>
            createTheme({
                palette: {
                    mode: prefersDarkMode ? 'dark' : 'light',
                },
            }),
        [prefersDarkMode],
    );

    return (
        <ThemeProvider theme={theme}>
            <CssBaseline/>
            <Page/>
        </ThemeProvider>
    );
}
