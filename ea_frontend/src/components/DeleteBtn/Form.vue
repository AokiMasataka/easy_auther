<template>
    <v-card
        :title="props.title"
        :subtitle="props.name"
        :text="props.id"
    >
        <v-card-actions class="flex mx-4 mb-4">
            <v-btn
                color="red"
                text="Delete"
                variant="outlined"
                @click="$emit('onDelete')"
            ></v-btn>

            <div class="grow"> </div>

            <v-btn
                text="Cancel"
                variant="outlined"
                @click="$emit('onClose')"
            ></v-btn>
        </v-card-actions>
    </v-card>
</template>

<script setup lang="ts">

const props = defineProps<{
    title: string,
    name: string,
    id: string
}>();

defineEmits<{
    (e: 'onClose'): void
    (e: 'onDelete'): void
}>();

</script>