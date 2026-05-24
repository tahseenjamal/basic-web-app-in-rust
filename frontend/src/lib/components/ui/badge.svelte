<script lang="ts">
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props {
    variant?: 'get' | 'post' | 'success' | 'error' | 'default';
    class?: string;
    children?: Snippet;
  }

  let { variant = 'default', class: className = '', children }: Props = $props();

  const variants: Record<string, string> = {
    get:     'bg-green-500/10 text-green-400 border-green-500/25',
    post:    'bg-blue-500/10 text-blue-400 border-blue-500/25',
    success: 'bg-green-500/10 text-green-400 border-green-500/25',
    error:   'bg-red-500/10 text-red-400 border-red-500/25',
    default: 'bg-muted text-muted-foreground border-border'
  };
</script>

<span
  class={cn(
    'inline-flex items-center rounded border px-2 py-0.5 text-xs font-bold font-mono tracking-wide',
    variants[variant],
    className
  )}
>
  {@render children?.()}
</span>
