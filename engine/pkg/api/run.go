package api

import (
	"context"
	"time"

	"github.com/Develonaut/bento/pkg/engine"
	"github.com/Develonaut/bento/pkg/logger"
	"github.com/Develonaut/bento/pkg/node"
)

// RunOptions configures a workflow execution.
type RunOptions struct {
	Timeout    time.Duration
	OnProgress func(nodeID, status string)
	Logger     *logger.Logger
}

// RunWorkflow executes a workflow definition and returns the result.
//
// The caller is responsible for file path resolution — this method
// accepts an already-loaded Definition. Timeout and cancellation are
// handled via the provided context; RunOptions.Timeout adds an
// additional deadline on top of the context.
func (s *BentoService) RunWorkflow(ctx context.Context, def *node.Definition, opts RunOptions) (*RunResult, error) {
	if err := s.validator.Validate(ctx, def); err != nil {
		return nil, err
	}

	if opts.Timeout > 0 {
		var cancel context.CancelFunc
		ctx, cancel = context.WithTimeout(ctx, opts.Timeout)
		defer cancel()
	}

	eng := engine.New(s.registry, opts.Logger)
	if opts.OnProgress != nil {
		eng.OnProgress(opts.OnProgress)
	}

	result, err := eng.Serve(ctx, def)
	return toRunResult(result, err), err
}

// DryRunWorkflow validates a workflow and reports what would execute.
//
// No nodes are actually run. Returns validation status plus a list of
// every node that would participate in execution.
func (s *BentoService) DryRunWorkflow(ctx context.Context, def *node.Definition) (*DryRunResult, error) {
	vr, err := s.ValidateWorkflow(ctx, def)
	if err != nil {
		return nil, err
	}

	nodes := collectNodes(def)

	return &DryRunResult{
		Valid:     vr.Valid,
		NodeCount: len(nodes),
		Nodes:     nodes,
	}, nil
}

// toRunResult translates an engine.Result into a serializable RunResult.
func toRunResult(r *engine.Result, err error) *RunResult {
	if r == nil {
		errMsg := ""
		if err != nil {
			errMsg = err.Error()
		}
		return &RunResult{Status: "failed", Error: errMsg}
	}

	rr := &RunResult{
		Status:        string(r.Status),
		NodesExecuted: r.NodesExecuted,
		NodeOutputs:   r.NodeOutputs,
		Duration:      r.Duration,
	}
	if r.Error != nil {
		rr.Error = r.Error.Error()
	}
	return rr
}

// collectNodes walks a Definition tree and returns info for every node.
func collectNodes(def *node.Definition) []NodeInfo {
	var nodes []NodeInfo
	for _, n := range def.Nodes {
		nodes = append(nodes, NodeInfo{
			ID:   n.ID,
			Name: n.Name,
			Type: n.Type,
		})
	}
	return nodes
}
