<template>
    <v-card
        :title="props.title"
    >
        <v-card-text>
            <v-text-field
                v-model="model.name"
                label="name"
            ></v-text-field>

            <v-text-field
                v-model="model.pass"
                label="pass"
            ></v-text-field>
        </v-card-text>

        <v-card-actions class="mx-4 mb-4">
            <v-btn
                text="Cancel"
                variant="outlined"
                @click="$emit('onClose')"
            ></v-btn>

            <v-btn
                color="primary"
                text="Create"
                variant="outlined"
                @click="$emit('onCreate')"
            ></v-btn>
        </v-card-actions>
    </v-card>
</template>

<script setup lang="ts">
import { AuthInfo } from '@/scripts/types';

const props = defineProps<{
    title: string,
}>();

const model = defineModel<AuthInfo>({ required: true });

defineEmits<{
    (e: 'onClose'): void,
    (e: 'onCreate'): void,
}>();

</script>