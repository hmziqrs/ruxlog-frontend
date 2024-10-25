'use client';
import {
  MDXEditor,
  headingsPlugin,
  listsPlugin,
  quotePlugin,
  thematicBreakPlugin,
  markdownShortcutPlugin,
  linkPlugin,
  linkDialogPlugin,
  imagePlugin,
  tablePlugin,
  toolbarPlugin,
  AdmonitionDirectiveDescriptor,
  DirectiveDescriptor,
  directivesPlugin,
  frontmatterPlugin,
  SandpackConfig,
  codeBlockPlugin,
  codeMirrorPlugin,
  sandpackPlugin,
  KitchenSinkToolbar,
  diffSourcePlugin,
} from '@mdxeditor/editor';
import '@mdxeditor/editor/style.css';
import { forwardRef, useCallback } from 'react';
import { useTheme } from 'next-themes';

interface EditorProps {
  markdown: string;
  onChange?: (value: string) => void;
  readOnly?: boolean;
  className?: string;
}

const defaultSnippetContent = `
export default function App() {
  return (
    <div className="App">
      <h1>Hello CodeSandbox</h1>
      <h2>Start editing to see some magic happen!</h2>
    </div>
  );
}
`.trim();

export const virtuosoSampleSandpackConfig: SandpackConfig = {
  defaultPreset: 'react',
  presets: [
    {
      label: 'React',
      name: 'react',
      meta: 'live react',
      sandpackTemplate: 'react',
      sandpackTheme: 'light',
      snippetFileName: '/App.js',
      snippetLanguage: 'jsx',
      initialSnippetContent: defaultSnippetContent,
    },
    {
      label: 'React',
      name: 'react',
      meta: 'live',
      sandpackTemplate: 'react',
      sandpackTheme: 'light',
      snippetFileName: '/App.js',
      snippetLanguage: 'jsx',
      initialSnippetContent: defaultSnippetContent,
    },
    {
      label: 'Virtuoso',
      name: 'virtuoso',
      meta: 'live virtuoso',
      sandpackTemplate: 'react-ts',
      sandpackTheme: 'light',
      snippetFileName: '/App.tsx',
      initialSnippetContent: defaultSnippetContent,
      dependencies: {
        'react-virtuoso': 'latest',
        '@ngneat/falso': 'latest',
      },
      files: {},
    },
  ],
};

export const YoutubeDirectiveDescriptor: DirectiveDescriptor<YoutubeDirectiveNode> =
  {
    name: 'youtube',
    type: 'leafDirective',
    testNode(node) {
      return node.name === 'youtube';
    },
    attributes: ['id'],
    hasChildren: false,
    Editor: ({ mdastNode, lexicalNode, parentEditor }) => {
      return (
        <div
          style={{
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'flex-start',
          }}
        >
          <button
            onClick={() => {
              parentEditor.update(() => {
                lexicalNode.selectNext();
                lexicalNode.remove();
              });
            }}
          >
            delete
          </button>
          <iframe
            width="560"
            height="315"
            src={`https://www.youtube.com/embed/${mdastNode.attributes.id}`}
            title="YouTube video player"
            frameBorder="0"
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
          ></iframe>
        </div>
      );
    },
  };

const InitializedMDXEditor = forwardRef<any, EditorProps>(
  ({ markdown, onChange, readOnly = false, className = '' }, ref) => {
    const { theme } = useTheme();
    const isDark = theme === 'dark';

    const handleImageUpload = useCallback(async (file: File) => {
      // TODO: Implement image upload
      // This is just a placeholder. You should implement your own image upload logic
      return URL.createObjectURL(file);
    }, []);

    return (
      <div className="relative w-full min-h-[200px] h-auto">
        <MDXEditor
          ref={ref}
          className={`w-full max-w-none prose dark:prose-invert ${className} ${
            isDark ? 'dark-theme' : ''
          }`}
          markdown={markdown}
          onChange={onChange}
          readOnly={readOnly}
          plugins={[
            toolbarPlugin({ toolbarContents: () => <KitchenSinkToolbar /> }),
            listsPlugin(),
            quotePlugin(),
            headingsPlugin(),
            linkPlugin(),
            linkDialogPlugin(),
            imagePlugin({
              imageAutocompleteSuggestions: [
                'https://via.placeholder.com/150',
                'https://via.placeholder.com/150',
              ],
              imageUploadHandler: async () =>
                Promise.resolve('https://picsum.photos/200/300'),
            }),
            tablePlugin(),
            thematicBreakPlugin(),
            frontmatterPlugin(),
            codeBlockPlugin({ defaultCodeBlockLanguage: '' }),
            sandpackPlugin({ sandpackConfig: virtuosoSampleSandpackConfig }),
            codeMirrorPlugin({
              codeBlockLanguages: {
                js: 'JavaScript',
                css: 'CSS',
                txt: 'Plain Text',
                tsx: 'TypeScript',
                rs: 'Rust',
                py: 'Python',
                go: 'Go',
                java: 'Java',
                sh: 'Shell',
                sql: 'SQL',
                '': 'Unspecified',
              },
            }),
            directivesPlugin({
              directiveDescriptors: [
                YoutubeDirectiveDescriptor,
                AdmonitionDirectiveDescriptor,
              ],
            }),
            diffSourcePlugin({ viewMode: 'rich-text', diffMarkdown: 'boo' }),
            markdownShortcutPlugin(),
          ]}
        />
      </div>
    );
  }
);

InitializedMDXEditor.displayName = 'InitializedMDXEditor';

export const Editor = forwardRef<any, EditorProps>((props, ref) => (
  <InitializedMDXEditor {...props} ref={ref} />
));

Editor.displayName = 'Editor';
