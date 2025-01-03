import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'

// Composables
import { createVuetify } from 'vuetify'
import colors from 'vuetify/util/colors'

export default createVuetify({
    theme: {
        themes: {
            dark: {
                colors: {
                primary: colors.purple.darken1,
                }
            },
        },
        defaultTheme: 'dark',
    },
})