<script lang="ts">
  import type { McpTool } from '#lib/utils/types.ts';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  async function loadTools() {
    try {
      const allTools = await invoke<McpTool[]>('get_all_tools');
      consoleLogTerminal(`${allTools[0].name}`);
    } catch (error) {
      console.error('Failed to fetch students:', error);
      consoleLogTerminal(`Failed to fetch students ${error}`);
    }
  }
  async function consoleLogTerminal(message: String) {
    await invoke('log_to_terminal', {
      message,
    });
  }

  onMount(() => {
    loadTools();
  });
</script>

<div class="tools-page"></div>
