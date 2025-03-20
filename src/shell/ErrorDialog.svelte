<script lang="ts">
    import ActionWidget from "../controls/ActionWidget.svelte";
    import Icon from "../controls/Icon.svelte";
    import ModalDialog from "./ModalDialog.svelte";

    interface Props {
        title: string;
        severe?: boolean;
        onClose?: (() => void) | null;
        children?: import('svelte').Snippet;
    }

    let {
        title,
        severe = false,
        onClose = null,
        children
    }: Props = $props();
</script>

<ModalDialog {title} error={severe} on:cancel={onClose}>
    {@render children?.()}

    {#snippet commands()}

            {#if onClose}
                <ActionWidget tip="close dialog" safe onClick={onClose}>OK</ActionWidget>
            {/if}

    {/snippet}
</ModalDialog>
