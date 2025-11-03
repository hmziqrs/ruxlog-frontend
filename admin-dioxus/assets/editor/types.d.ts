// Type declarations for Editor.js packages without official types

declare module 'editorjs-drag-drop' {
  import type EditorJS from '@editorjs/editorjs';

  export default class DragDrop {
    constructor(editor: EditorJS);
  }
}

declare module 'editorjs-undo' {
  import type EditorJS from '@editorjs/editorjs';

  interface UndoConfig {
    editor: EditorJS;
    maxLength?: number;
    onUpdate?: () => void;
  }

  export default class Undo {
    constructor(config: UndoConfig);
    initialize(data: any): void;
    clear(): void;
    count(): number;
  }
}

declare module 'editorjs-button' {
  import type { BlockTool, BlockToolConstructorOptions } from '@editorjs/editorjs';

  export default class Button implements BlockTool {
    constructor(options: BlockToolConstructorOptions);
    static get toolbox(): { icon: string; title: string };
    render(): HTMLElement;
    save(block: HTMLElement): any;
  }
}

declare module 'editorjs-alert' {
  import type { BlockTool, BlockToolConstructorOptions } from '@editorjs/editorjs';

  export default class Alert implements BlockTool {
    constructor(options: BlockToolConstructorOptions);
    static get toolbox(): { icon: string; title: string };
    render(): HTMLElement;
    save(block: HTMLElement): any;
  }
}
