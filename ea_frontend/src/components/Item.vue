<template>
    <tr class="bg-neutral-900 border-b">
        <td class="px-6 py-4 dark:text-gray-300">
            <a class="hover:cursor-pointer hover:border-b" @click='$emit("onClick")'>
                {{ props.name }}
            </a>
        </td>
        <td class="px-6 py-4 dark:text-gray-300">{{ props.email }}</td>
        <td class="px-6 py-2 dark:text-gray-300">
            <Trash
                class="w-5 h-5 hover:cursor-pointer hover:text-red-600"
                @click.stop='$emit("onDelete")'
            />
        </td>
    </tr>
</template>
<script setup lang="ts">
import Trash from '@/components/icons/Trash.vue';

const props = defineProps<{
    id: string;
    name: string;
    email: string;
}>();

defineEmits<{
    (e: 'onClick'): void,
    (e: 'onDelete'): void,
}>();
</script>