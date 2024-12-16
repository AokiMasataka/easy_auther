<template>
    <v-dialog v-model="model" width="800">
        <v-card
            title="Your secret key"
        >
            <v-card-text>
                <v-textarea
                    label=""
                    :model-value="props.pem"
                    name="input-7-1"
                
                    variant="outlined"
                    auto-grow
                    readonly
                />
            </v-card-text>

            <v-card-actions>
                <v-btn
                    text="Close"
                    @click="model = false"
                />
            </v-card-actions>
        </v-card>
    </v-dialog>

</template>

<script setup lang="ts">
const props = defineProps<{pem: string}>();
const model = defineModel<boolean>();

</script>