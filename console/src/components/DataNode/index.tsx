import React, { memo, FC, useCallback, ReactElement, useEffect } from 'react';
import { Box } from '@mui/material';
import {
  NodeProps,
  Handle,
  HandleType,
  Position,
  useStore,
  useReactFlow,
  getConnectedEdges,
} from 'reactflow';
import styles from './styles.js';
import DataChip from '../DataChip.tsx';
import CloseButton from '../CloseButton.tsx';
import { toTitleCase } from '../../utils/index.ts';
import useFlowStore, { selector } from '../../stores/flowStore.tsx';
import { useTheme } from '@mui/material/styles';
interface DataNodeProps extends NodeProps {
  data: {
    display_name: string;
    id: string;
    type: string;
    clientId: string;
    clientName: string;
    source: boolean;
    destination: boolean;
    password: string;
    orphan: boolean;
  };
}

const renderHandle = (handleType: string): ReactElement => {
  const theme = useTheme();
  const typeColor = handleType === 'source' ? theme.palette.forest.main : theme.palette.forest.dark;
  return (
    <Handle
      type={handleType as HandleType}
      id={handleType}
      position={handleType === 'source' ? Position.Right : Position.Left}
      isConnectable
      style={{
        ...styles.handle,
        background: typeColor,
        border: `1px solid ${typeColor}`,
      }}
    />
  );
};

const DataNode: FC<DataNodeProps> = memo(function DataNode(props) {
  const { id, data } = props;
  const theme = useTheme();
  const rf = useReactFlow();
  const {
    addEdgeToBeDeleted,
    setActiveNode,
    addNodeToBeDeleted,
    edges,
    getNode,
    addUnconnectedNode,
  } = useFlowStore(selector);

  const hasConnection = useStore((s) =>
    s.edges.some((edge) => edge.source === id || edge.target === id)
  );
  useEffect(() => {
    if (!hasConnection) addUnconnectedNode(id);
  }, [hasConnection]);

  const onRemove = useCallback(() => {
    if (confirm('Are you sure you want to delete this node?')) {
      const deleted = getNode(id);
      if (!deleted) return;
      const connected = getConnectedEdges([deleted], edges);
      if (connected.length) connected.forEach((e) => addEdgeToBeDeleted(e.data.id));

      // this following removed node from the UI.
      // The Publish button in the Flow component
      // handles the API call to delete
      rf.deleteElements({ nodes: [{ id }] });
      addNodeToBeDeleted(deleted);
      setActiveNode(null);
    }
  }, [id]);

  return (
    <Box
      className="gradient"
      sx={{
        ...styles.gradient,
        border: `${props.selected ? '2.5px solid transparent' : '1.5px solid transparent'}`,
        boxShadow: `${props.selected ? '2px 2px 2px #dadada' : 'none'}`,
      }}
    >
      <Box
        className="dataNode"
        sx={{
          ...styles.node,
          boxShadow: `${props.selected ? `0 0 0 0.5px ${theme.palette.forest.dark}` : 'none'}`,
          bgcolor: `${hasConnection ? 'white' : '#dadada'}`,
        }}
      >
        {data.destination && renderHandle('target')}
        {data.source && renderHandle('source')}
        <Box>
          <Box>
            {data.source && <DataChip flowType="source" />}
            {data.destination && <DataChip flowType="destination" />}
            <CloseButton onClick={onRemove} />
          </Box>
          <br />
          <Box sx={{ display: 'flex', flexDirection: 'column', width: '100%' }} my={1}>
            <Box sx={{ fontSize: '0.6rem' }}>{toTitleCase(data.clientName)}</Box>
            <Box role="contentinfo" sx={{ fontSize: '0.9rem', color: 'forest.dark' }}>
              {toTitleCase(data.display_name)}
            </Box>
          </Box>
        </Box>
      </Box>
    </Box>
  );
});

export default memo(DataNode);