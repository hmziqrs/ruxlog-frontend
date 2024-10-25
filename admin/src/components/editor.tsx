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
  UndoRedo,
  BoldItalicUnderlineToggles,
  BlockTypeSelect,
  CreateLink,
  InsertImage,
  InsertTable,
  InsertThematicBreak,
  ListsToggle,
  CodeToggle,
  diffSourcePlugin,
  DiffSourceToggleWrapper,
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
      <div className="relative w-full">
        <MDXEditor
          ref={ref}
          className={`w-full prose dark:prose-invert ${className} ${
            isDark ? 'dark-theme' : ''
          }`}
          markdown={markdown}
          onChange={onChange}
          readOnly={readOnly}
          plugins={[
            headingsPlugin(),
            listsPlugin(),
            quotePlugin(),
            thematicBreakPlugin(),
            linkPlugin(),
            linkDialogPlugin(),
            imagePlugin({ imageUploadHandler: handleImageUpload }),
            tablePlugin(),
            diffSourcePlugin(),
            markdownShortcutPlugin(),
            toolbarPlugin({
              toolbarContents: () => (
                <DiffSourceToggleWrapper>
                  <div className="flex flex-wrap gap-2 p-1">
                    <UndoRedo />
                    <BoldItalicUnderlineToggles />
                    <BlockTypeSelect />
                    <CreateLink />
                    <InsertImage />
                    <InsertTable />
                    <InsertThematicBreak />
                    <ListsToggle />
                    <CodeToggle />
                  </div>
                </DiffSourceToggleWrapper>
              ),
            }),
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
