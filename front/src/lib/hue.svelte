<script context="module" lang="ts">
    export function colorFromHue(hue: number) {
        return `hsl(${hue}, 50%, 50%)`;
    }

    export function now() {
        return Math.floor(new Date().getTime() / 1000);
    }

    export function hueFromTime(now: number) {
        return now % 360;
    }
</script>
