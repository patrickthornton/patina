<script context="module" lang="ts">
    export interface Post {
        post_id: number;
        author: string;
        text: string;
        hue: number;
        reply_to?: number;
        likes: number;
        timestamp: number;
    }

    export interface User {
        user_id: number;
        name: string;
    }
</script>
