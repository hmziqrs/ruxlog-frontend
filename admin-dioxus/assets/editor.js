const READY_EVENT = 'DOMContentLoaded';
const CHANGE_EVENT = 'editor:change';
const SAVE_EVENT = 'editor:save';
const HOLDER_ID = 'editorjs';

const CDN = {
  editor: 'https://esm.sh/@editorjs/editorjs@2.30.7',
  header: 'https://esm.sh/@editorjs/header@2.8.1',
  list: 'https://esm.sh/@editorjs/list@1.9.0',
  image: 'https://esm.sh/@editorjs/image@2.8.1',
  code: 'https://esm.sh/@editorjs/code@2.8.0',
  quote: 'https://esm.sh/@editorjs/quote@2.7.2',
};

const defaultSeed = () => ({
  blocks: [
    {
      type: 'header',
      data: {
        text: 'Welcome to Editor.js!',
        level: 2,
      },
    },
    {
      type: 'paragraph',
      data: {
        text: 'Start writing your blog post here…',
      },
    },
  ],
});

const ready = (fn) => {
  if (document.readyState === 'loading') {
    document.addEventListener(READY_EVENT, fn, { once: true });
  } else {
    fn();
  }
};

const parseInitialData = () => {
  const raw = window.__EDITOR_INITIAL_DATA_RAW;
  if (!raw) {
    return defaultSeed();
  }

  try {
    const parsed = JSON.parse(raw);
    if (parsed && typeof parsed === 'object') {
      return parsed;
    }
  } catch (err) {
    console.error('[EditorJS] Failed to parse initial data, falling back to default', err);
  }

  return defaultSeed();
};

const emitEvent = (name, detail) => {
  try {
    window.dispatchEvent(new CustomEvent(name, { detail }));
  } catch (err) {
    console.error('[EditorJS] Failed to emit event', name, err);
  }
};

const withStringDetail = (data) => {
  try {
    return JSON.stringify(data);
  } catch (err) {
    console.error('[EditorJS] Failed to stringify data', err);
    return null;
  }
};

const loadModule = async (url, name) => {
  try {
    return await import(/* @vite-ignore */ url);
  } catch (err) {
    throw new Error(`[EditorJS] Failed to load ${name} from ${url}: ${err}`);
  }
};

const ensureDependencies = async () => {
  if (window.__editorjs_cdn_loading) {
    return window.__editorjs_cdn_loading;
  }

  window.__editorjs_cdn_loading = (async () => {
    const [EditorJSMod, HeaderMod, ListMod, imageModule, CodeMod, QuoteMod] = await Promise.all([
      loadModule(CDN.editor, 'EditorJS core'),
      loadModule(CDN.header, 'Header tool'),
      loadModule(CDN.list, 'List tool'),
      loadModule(CDN.image, 'Image tool'),
      loadModule(CDN.code, 'Code tool'),
      loadModule(CDN.quote, 'Quote tool'),
    ]);

    const EditorJS = EditorJSMod?.default || EditorJSMod?.EditorJS;
    const Header = HeaderMod?.default || HeaderMod?.Header;
    const List = ListMod?.default || ListMod?.List;
    const ImageTool =
      imageModule?.default || imageModule?.ImageTool || imageModule?.SimpleImage;
    const CodeTool = CodeMod?.default || CodeMod?.CodeTool;
    const Quote = QuoteMod?.default || QuoteMod?.Quote;

    const missing = [];
    if (!EditorJS) missing.push('EditorJS core');
    if (!Header) missing.push('Header tool');
    if (!List) missing.push('List tool');
    if (!ImageTool) missing.push('Image tool');
    if (!CodeTool) missing.push('Code tool');
    if (!Quote) missing.push('Quote tool');

    if (missing.length) {
      throw new Error(`[EditorJS] Missing dependencies: ${missing.join(', ')}`);
    }

    Object.assign(window, {
      EditorJS,
      Header,
      List,
      ImageTool,
      SimpleImage: imageModule?.SimpleImage || ImageTool,
      CodeTool,
      Quote,
    });
  })();

  return window.__editorjs_cdn_loading;
};

const mountEditor = async () => {
  await ensureDependencies();

  if (window.__EDITORJS_INSTANCE__ && typeof window.__EDITORJS_INSTANCE__.destroy === 'function') {
    window.__EDITORJS_INSTANCE__.destroy();
  }

  const editor = new window.EditorJS({
    holder: HOLDER_ID,
    autofocus: false,
    logLevel: 'ERROR',
    placeholder: 'Start writing your blog post…',
    tools: {
      header: window.Header,
      list: window.List,
      image: {
        class: window.ImageTool || window.SimpleImage,
        config: {
          uploader: {
            uploadByUrl: async (url) => ({ success: 1, file: { url } }),
          },
        },
      },
      code: window.CodeTool,
      quote: window.Quote,
    },
    data: parseInitialData(),
    onReady: async () => {
      emitEvent('editor:ready', null);
      try {
        const data = await editor.save();
        const detail = withStringDetail(data);
        if (detail) {
          emitEvent(CHANGE_EVENT, detail);
        }
      } catch (err) {
        console.error('[EditorJS] Initial save failed', err);
      }
    },
    onChange: async () => {
      try {
        const data = await editor.save();
        const detail = withStringDetail(data);
        if (detail) {
          emitEvent(CHANGE_EVENT, detail);
        }
      } catch (err) {
        console.error('[EditorJS] onChange save failed', err);
      }
    },
  });

  window.__EDITORJS_INSTANCE__ = editor;

  window.editor = {
    save: async () => editor.save(),
    clear: async () => editor.clear(),
    render: async (data) => editor.render(data),
    destroy: () => editor.destroy(),
    instance: editor,
  };

  window.addEventListener('editor:request-save', async () => {
    try {
      const data = await editor.save();
      const detail = withStringDetail(data);
      if (detail) {
        emitEvent(SAVE_EVENT, detail);
      }
    } catch (err) {
      console.error('[EditorJS] Forced save failed', err);
    }
  });
};

ready(() => {
  mountEditor().catch((err) => {
    console.error(err);
  });
});
