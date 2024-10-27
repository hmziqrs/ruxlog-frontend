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
  DirectiveNode,
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

interface YoutubeDirectiveNode {
  type: 'leafDirective';
  name: string;
  attributes: {
    id: string;
  };
  children: any[];
}

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
            style={{ border: 0 }}
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
      <div className="border rounded-md">
        <MDXEditor
          ref={ref}
          className={`w-full h-full max-w-none prose dark:prose-invert ${className} ${
            isDark ? 'dark-theme' : ''
          }`}
          contentEditableClassName="min-h-[200px] sm:min-h-[260px] md:min-h-[320px]"
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
            // sandpackPlugin(),
            // sandpackPlugin({ sandpackConfig: virtuosoSampleSandpackConfig }),
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
