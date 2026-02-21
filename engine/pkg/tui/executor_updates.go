package tui

import "time"

// handleNodeStarted updates node to running state.
func (e *Executor) handleNodeStarted(msg NodeStartedMsg) {
	for i := range e.nodeStates {
		if e.nodeStates[i].path == msg.Path {
			e.nodeStates[i].status = NodeRunning
			e.nodeStates[i].startTime = time.Now()
			return
		}
	}
}

// handleNodeCompleted updates node to completed/failed state.
func (e *Executor) handleNodeCompleted(msg NodeCompletedMsg) {
	for i := range e.nodeStates {
		if e.nodeStates[i].path == msg.Path {
			e.nodeStates[i].duration = msg.Duration
			if msg.Error != nil {
				e.nodeStates[i].status = NodeFailed
			} else {
				e.nodeStates[i].status = NodeCompleted
			}
			// Clear loop child info when node completes
			e.nodeStates[i].currentChild = ""
			return
		}
	}
}

// handleLoopChild updates loop with current child execution info.
func (e *Executor) handleLoopChild(msg LoopChildMsg) {
	for i := range e.nodeStates {
		if e.nodeStates[i].path == msg.LoopPath {
			e.nodeStates[i].currentChild = msg.ChildName
			e.nodeStates[i].childIndex = msg.Index
			e.nodeStates[i].childTotal = msg.Total
			return
		}
	}
}

// updateSequence converts node states to sequence steps.
func (e *Executor) updateSequence() {
	steps := make([]Step, len(e.nodeStates))
	for i, node := range e.nodeStates {
		steps[i] = Step{
			Name:         node.name,
			Type:         node.nodeType,
			Status:       convertNodeStatusToStepStatus(node.status),
			Duration:     node.duration,
			Depth:        node.depth,
			CurrentChild: node.currentChild,
			ChildIndex:   node.childIndex,
			ChildTotal:   node.childTotal,
		}
	}
	e.sequence.SetSteps(steps)
}

// convertNodeStatusToStepStatus converts NodeStatus to StepStatus.
func convertNodeStatusToStepStatus(status NodeStatus) StepStatus {
	switch status {
	case NodePending:
		return StepPending
	case NodeRunning:
		return StepRunning
	case NodeCompleted:
		return StepCompleted
	case NodeFailed:
		return StepFailed
	default:
		return StepPending
	}
}

// updateProgress calculates and updates the progress bar percentage.
func (e *Executor) updateProgress() {
	if len(e.nodeStates) == 0 {
		e.progress.SetPercent(0.0)
		return
	}

	completed := 0
	for _, node := range e.nodeStates {
		if node.status == NodeCompleted || node.status == NodeFailed {
			completed++
		}
	}

	percent := float64(completed) / float64(len(e.nodeStates))
	e.progress.SetPercent(percent)
}
