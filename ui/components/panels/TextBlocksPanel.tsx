'use client'

import { CheckSquare2Icon, Languages, LoaderCircleIcon, Trash2Icon, XIcon } from 'lucide-react'
import { motion } from 'motion/react'
import { useEffect, useMemo, useRef, useState } from 'react'
import { useTranslation } from 'react-i18next'

import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/accordion'
import { Button } from '@/components/ui/button'
import { DraftTextarea } from '@/components/ui/draft-textarea'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import { useCurrentPage, useTextNodes, type TextNodeEntry } from '@/hooks/useCurrentPage'
import { useMarqueeSelection } from '@/hooks/useMarqueeSelection'
import { getConfig, startPipeline, useGetCurrentLlm } from '@/lib/api/default/default'
import type { TextDataPatch } from '@/lib/api/schemas'
import { applyOp, queueAutoRender, reorderPageTextNodes } from '@/lib/io/scene'
import { ops } from '@/lib/ops'
import { useEditorUiStore } from '@/lib/stores/editorUiStore'
import { useJobsStore } from '@/lib/stores/jobsStore'
import { usePreferencesStore } from '@/lib/stores/preferencesStore'
import { useSelectionStore } from '@/lib/stores/selectionStore'
import { cn } from '@/lib/utils'

export function TextBlocksPanel() {
  const { t } = useTranslation()
  const page = useCurrentPage()
  const textNodes = useTextNodes()
  useEffect(() => {
    if (process.env.NODE_ENV !== 'production') {
      console.debug(
        '[reorder] Text nodes order:',
        textNodes.map((n) => n.id),
      )
    }
  }, [textNodes])
  const selectedIds = useSelectionStore((s) => s.nodeIds)
  const select = useSelectionStore((s) => s.select)
  const selectMany = useSelectionStore((s) => s.selectMany)
  const clearSelection = useSelectionStore((s) => s.clear)
  const viewportRef = useRef<HTMLDivElement | null>(null)
  const [openNodeId, setOpenNodeId] = useState<string | null>(null)
  const { data: llm } = useGetCurrentLlm()
  const llmReady = llm?.status === 'ready'
  const isProcessing = useJobsStore((s) =>
    Object.values(s.jobs).some((j) => j.status === 'running'),
  )
  const readingOrder = useEditorUiStore((s) => s.readingOrder)
  const setReadingOrder = useEditorUiStore((s) => s.setReadingOrder)

  const selectedTextIds = useMemo(() => {
    const textIds = new Set(textNodes.map((node) => node.id))
    return new Set([...selectedIds].filter((id) => textIds.has(id)))
  }, [selectedIds, textNodes])
  const { marqueeRect, marqueeHandlers } = useMarqueeSelection({
    viewportRef,
    selectedIds: selectedTextIds,
    onSelectMany: selectMany,
    onClear: clearSelection,
  })

  useEffect(() => {
    if (marqueeRect) return
    if (selectedTextIds.size === 1) setOpenNodeId([...selectedTextIds][0])
    if (selectedTextIds.size === 0) setOpenNodeId(null)
  }, [marqueeRect, selectedTextIds])

  if (!page) {
    return (
      <div className='flex flex-1 items-center justify-center text-xs text-muted-foreground'>
        {t('textBlocks.emptyPrompt')}
      </div>
    )
  }

  const openIndex = textNodes.findIndex((node) => node.id === openNodeId)
  const accordionValue = openIndex >= 0 ? openIndex.toString() : ''

  const patchText = async (nodeId: string, patch: TextDataPatch) => {
    const editEpoch = await applyOp(
      ops.updateNode(page.id, nodeId, {
        data: { text: patch } as never,
      }),
    )
    queueAutoRender(page.id, editEpoch)
  }

  const removeNode = async (nodeId: string) => {
    const node = page.nodes[nodeId]
    if (!node) return
    const idx = Object.keys(page.nodes).indexOf(nodeId)
    const editEpoch = await applyOp(ops.removeNode(page.id, nodeId, node, idx < 0 ? 0 : idx))
    clearSelection()
    queueAutoRender(page.id, editEpoch)
  }

  const removeNodes = async (nodeIds: string[]) => {
    const batch = nodeIds.flatMap((nodeId) => {
      const node = page.nodes[nodeId]
      if (!node) return []
      const idx = Object.keys(page.nodes).indexOf(nodeId)
      return [ops.removeNode(page.id, nodeId, node, idx < 0 ? 0 : idx)]
    })
    const editEpoch = await applyOp(ops.batch('removeNodes', batch))
    clearSelection()
    queueAutoRender(page.id, editEpoch)
  }

  const generate = async (nodeId: string) => {
    if (!page) return
    const cfg = await getConfig()
    const translator = cfg.pipeline?.translator || 'llm'
    const renderer = cfg.pipeline?.renderer || 'yomika-renderer'
    const editor = useEditorUiStore.getState()
    const prefs = usePreferencesStore.getState()
    // Keep rendering page-scoped, but constrain translation to the clicked block.
    await startPipeline({
      steps: [translator, renderer],
      pages: [page.id],
      textNodeIds: [nodeId],
      targetLanguage: editor.selectedLanguage,
      systemPrompt: prefs.customSystemPrompt,
      defaultFont: prefs.defaultFont,
      readingOrder: editor.readingOrder === 'custom' ? undefined : editor.readingOrder,
    })
  }

  return (
    <div className='flex min-h-0 flex-1 flex-col' data-testid='panels-textblocks'>
      <div className='flex items-center justify-between border-b border-border px-2 py-1.5 text-xs font-semibold tracking-wide text-muted-foreground uppercase'>
        <span data-testid='textblocks-count' data-count={textNodes.length}>
          {t('textBlocks.title', { count: textNodes.length })}
        </span>
        <div className='flex items-center gap-1.5'>
          <span className='font-normal uppercase opacity-50'>{t('textBlocks.readingOrder')}:</span>
          <Select
            value={readingOrder}
            onValueChange={async (val: 'rtl' | 'ltr' | 'custom') => {
              if (process.env.NODE_ENV !== 'production') {
                console.debug('[reorder] Changing reading order to:', val)
              }

              if (val === 'custom') {
                setReadingOrder(val)
                return
              }

              try {
                await reorderPageTextNodes(page.id, val)
                setReadingOrder(val)
              } catch (err) {
                console.error('[reorder] Failed to reorder text nodes:', err)
                useEditorUiStore.getState().showError(String(err))
              }
            }}
          >
            <SelectTrigger
              className='h-5 w-32 gap-1 border-none bg-transparent px-1.5 text-[10px] font-semibold uppercase hover:bg-accent focus:ring-0'
              aria-label={t('textBlocks.readingOrder')}
            >
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value='rtl' className='text-[10px] font-semibold'>
                {t('textBlocks.readingOrderRtl')}
              </SelectItem>
              <SelectItem value='ltr' className='text-[10px] font-semibold'>
                {t('textBlocks.readingOrderLtr')}
              </SelectItem>
              <SelectItem value='custom' className='text-[10px] font-semibold'>
                {t('textBlocks.readingOrderCustom')}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>
      <ScrollArea
        key={page.id}
        className='min-h-0 flex-1'
        viewportClassName='pb-1'
        viewportRef={viewportRef}
        data-testid='textblocks-scroll'
        {...marqueeHandlers}
      >
        <div
          data-testid='textblocks-marquee-surface'
          className={cn('relative min-h-full p-2', marqueeRect && 'select-none')}
        >
          {textNodes.length === 0 ? (
            <p className='rounded-md border border-dashed border-border p-2 text-xs text-muted-foreground'>
              {t('textBlocks.none')}
            </p>
          ) : (
            <Accordion
              data-testid='textblocks-accordion'
              type='single'
              collapsible
              value={accordionValue}
              onValueChange={(value) => {
                if (!value) {
                  setOpenNodeId(null)
                  return
                }
                const idx = Number(value)
                const node = textNodes[idx]
                if (node) {
                  setOpenNodeId(node.id)
                  select(node.id, false)
                }
              }}
              className='flex flex-col gap-1'
            >
              {textNodes.map((node, index) => (
                <BlockCard
                  key={node.id}
                  node={node}
                  index={index}
                  selected={selectedIds.has(node.id)}
                  onToggleSelect={() => select(node.id, true)}
                  onPatch={(patch) => void patchText(node.id, patch)}
                  onDelete={() => void removeNode(node.id)}
                  onGenerate={() => void generate(node.id)}
                  processing={isProcessing}
                  llmReady={llmReady}
                />
              ))}
            </Accordion>
          )}
          {textNodes.length > 0 && (
            <p className='pointer-events-none mt-3 text-center text-[10px] text-muted-foreground/70'>
              {t('textBlocks.marqueeHint')}
            </p>
          )}
          {marqueeRect && (
            <div
              data-testid='textblocks-marquee'
              className='pointer-events-none absolute z-50 rounded-sm border border-primary bg-primary/15'
              style={marqueeRect}
            />
          )}
        </div>
      </ScrollArea>
      <div
        className={cn(
          'flex items-center justify-between border-t border-border px-2 py-1.5 text-xs font-semibold tracking-wide text-muted-foreground',
          textNodes.length === 0 && 'hidden',
        )}
      >
        <span className='text-[10px] tabular-nums'>
          {t('textBlocks.selectedCount', { count: selectedTextIds.size })}
        </span>
        <div className='flex items-center gap-1'>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                data-testid='textblocks-select-all'
                aria-label={t('textBlocks.selectAll')}
                variant='ghost'
                size='icon-xs'
                className='size-6'
                onClick={() => selectMany(textNodes.map((node) => node.id))}
              >
                <CheckSquare2Icon className='size-3.5' />
              </Button>
            </TooltipTrigger>
            <TooltipContent side='left' sideOffset={4}>
              {t('textBlocks.selectAll')}
            </TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                data-testid='textblocks-clear-selection'
                aria-label={t('textBlocks.clearSelection')}
                variant='ghost'
                size='icon-xs'
                className='size-6'
                disabled={selectedTextIds.size === 0}
                onClick={clearSelection}
              >
                <XIcon className='size-3.5' />
              </Button>
            </TooltipTrigger>
            <TooltipContent side='left' sideOffset={4}>
              {t('textBlocks.clearSelection')}
            </TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                data-testid='textblocks-delete-selected'
                aria-label={t('workspace.deleteSelected')}
                variant='ghost'
                size='icon-xs'
                className='size-6 text-rose-600 hover:text-rose-600'
                disabled={selectedTextIds.size === 0 || isProcessing}
                onClick={() => void removeNodes([...selectedTextIds])}
              >
                <Trash2Icon className='size-3.5' />
              </Button>
            </TooltipTrigger>
            <TooltipContent side='left' sideOffset={4}>
              {t('workspace.deleteSelected')}
            </TooltipContent>
          </Tooltip>
        </div>
      </div>
    </div>
  )
}

type BlockCardProps = {
  node: TextNodeEntry
  index: number
  selected: boolean
  onToggleSelect: () => void
  onPatch: (patch: TextDataPatch) => void
  onDelete: () => void
  onGenerate: () => void
  processing: boolean
  llmReady: boolean
}

function BlockCard({
  node,
  index,
  selected,
  onToggleSelect,
  onPatch,
  onDelete,
  onGenerate,
  processing,
  llmReady,
}: BlockCardProps) {
  const { t } = useTranslation()
  const data = node.data
  const hasOcr = !!data.text?.trim()
  const hasTranslation = !!data.translation?.trim()
  const preview = data.translation?.trim() || data.text?.trim()

  return (
    <motion.div
      data-testid={`textblock-card-${index}`}
      data-textblock-item
      data-textblock-id={node.id}
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.2, delay: index * 0.03 }}
    >
      <AccordionItem
        value={index.toString()}
        data-selected={selected}
        className='overflow-hidden rounded-md bg-card/90 text-xs ring-1 ring-border data-[selected=true]:ring-primary'
      >
        <AccordionTrigger
          data-testid={`textblock-trigger-${index}`}
          onClick={(e) => {
            if (e.shiftKey || e.ctrlKey || e.metaKey) {
              e.preventDefault()
              e.stopPropagation()
              onToggleSelect()
            }
          }}
          className='flex w-full cursor-pointer items-center gap-1.5 px-2 py-1.5 text-left transition outline-none hover:no-underline data-[state=open]:bg-accent [&>svg]:hidden'
        >
          <span
            className={`shrink-0 rounded-md px-1.5 py-0.5 text-center text-[10px] font-medium text-white tabular-nums ${
              selected ? 'bg-primary' : 'bg-muted-foreground/60'
            }`}
            style={{ minWidth: '1.5rem' }}
          >
            {index + 1}
          </span>
          <div className='flex min-w-0 flex-1 items-center gap-1'>
            <span
              className={`shrink-0 rounded-sm px-1 py-0.5 text-[9px] font-medium uppercase ${
                hasOcr ? 'bg-rose-400/70 text-white' : 'bg-muted text-muted-foreground/50'
              }`}
            >
              {t('textBlocks.ocrBadge')}
            </span>
            <span
              className={`shrink-0 rounded-sm px-1 py-0.5 text-[9px] font-medium uppercase ${
                hasTranslation ? 'bg-rose-400/70 text-white' : 'bg-muted text-muted-foreground/50'
              }`}
            >
              {t('textBlocks.translationBadge')}
            </span>
            {preview && (
              <p className='line-clamp-1 min-w-0 flex-1 text-xs text-muted-foreground'>{preview}</p>
            )}
          </div>
        </AccordionTrigger>
        <AccordionContent className='px-2 pt-1.5 pb-2 shadow-[inset_0_1px_0_0_var(--color-border)]'>
          <div className='space-y-1.5'>
            <div className='flex flex-col gap-0.5'>
              <span className='text-[10px] text-muted-foreground uppercase'>
                {t('textBlocks.ocrLabel')}
              </span>
              <DraftTextarea
                data-testid={`textblock-ocr-${index}`}
                value={data.text ?? ''}
                placeholder={t('textBlocks.addOcrPlaceholder')}
                rows={2}
                onValueChange={(value) => onPatch({ text: value })}
                className='min-h-0 resize-none px-1.5 py-1 text-xs'
              />
            </div>
            <div className='flex flex-col gap-0.5'>
              <div className='flex items-center justify-between'>
                <span className='text-[10px] text-muted-foreground uppercase'>
                  {t('textBlocks.translationLabel')}
                </span>
                <div className='flex items-center gap-0.5'>
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <Button
                        data-testid={`textblock-delete-${index}`}
                        aria-label={t('workspace.deleteBlock')}
                        variant='ghost'
                        size='icon-xs'
                        disabled={processing}
                        onClick={onDelete}
                        className='size-5 text-rose-600 hover:text-rose-600'
                      >
                        <Trash2Icon className='size-3' />
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent side='left' sideOffset={4}>
                      {t('workspace.deleteBlock')}
                    </TooltipContent>
                  </Tooltip>
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <Button
                        data-testid={`textblock-generate-${index}`}
                        aria-label={t('llm.generateTooltip')}
                        variant='ghost'
                        size='icon-xs'
                        disabled={!llmReady || processing}
                        onClick={onGenerate}
                        className='size-5'
                      >
                        {processing ? (
                          <LoaderCircleIcon className='size-3 animate-spin' />
                        ) : (
                          <Languages className='size-3' />
                        )}
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent side='left' sideOffset={4}>
                      {t('llm.generateTooltip')}
                    </TooltipContent>
                  </Tooltip>
                </div>
              </div>
              <DraftTextarea
                data-testid={`textblock-translation-${index}`}
                value={data.translation ?? ''}
                placeholder={t('textBlocks.addTranslationPlaceholder')}
                rows={2}
                onValueChange={(value) => onPatch({ translation: value })}
                className='min-h-0 resize-none px-1.5 py-1 text-xs'
              />
            </div>
          </div>
        </AccordionContent>
      </AccordionItem>
    </motion.div>
  )
}
