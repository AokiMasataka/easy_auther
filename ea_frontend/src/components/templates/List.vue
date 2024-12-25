<template>
    <v-data-table :items="props.items" :headers="headers">
        <template v-slot:item.action="{ item }">
            <a :href="uri(item.id)">
            <v-icon
                class="mr-3"
                size="small"
            >mdi-pencil</v-icon>
            </a>
            <DeleteBtn
                title="Delete?"
                :name="item.name"
                :id="item.id"
                @on-delete="$emit('onDelete', item.id)"
            />
        </template>
    </v-data-table>
</template>

<script setup lang="ts">
import { ItemInfo } from '../../scripts/types';
import DeleteBtn from './DeleteBtn.vue';

const headers = [
    {title: "ID", key: "id"},
    {title: "Name", key: "name"},
    {title: " ", key: "action", sortable: false}
];

const props = defineProps<{
    items: ItemInfo[],
    uri: (id: string) => string
}>();

defineEmits<{
    (e: 'onDelete', id: string): void,
}>();

</script>
