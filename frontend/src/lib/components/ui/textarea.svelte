<script lang="ts">
  import { cn } from '$lib/utils';

  let {
    class: className = '',
    value = $bindable(''),
    ...rest
  } = $props();
</script>

<textarea
  bind:value
  class={cn(
    'flex min-h-[80px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm',
    'transition-colors placeholder:text-muted-foreground resize-none',
    'focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring',
    'disabled:cursor-not-allowed disabled:opacity-50',
    className
  )}
  {...rest}
></textarea>
