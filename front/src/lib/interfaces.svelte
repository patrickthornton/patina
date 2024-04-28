<script context="module" lang="ts">
    export interface Post {
        id: number;
        author: string;
        text: string;
        hue: number;
        reply_to?: number;
        likes: number;
        timestamp: number;
    }
</script>
