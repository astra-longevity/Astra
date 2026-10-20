# Parent model: Farrell–Rutenberg network models of ageing (2016–2022)

Role in Astra: the parent of a whole-body view in which organ systems damage each other, as opposed to the single damage variable of the SR model (models/01). Related: A18 frailty index (mouse/01), anchors.md.

## What it models

Two generations from Rutenberg's group:
- **Network model (Taneja 2016; Farrell 2016).** Each person is a network of nodes, one per health attribute. Nodes become damaged (a health deficit) at random; the damage rate of a node depends on whether its neighbours are damaged, which stands for interactions between body systems. The frailty index is the share of damaged nodes in a chosen subset (Farrell 2018 abstract). Mortality is also read off the network (Taneja 2016, title: "health deficits and mortality").
- **DJIN (Farrell 2022).** A dynamic, joint, interpretable network model. Health variables are continuous (physical, functional and molecular), coupled by explicit pairwise interactions inside a stochastic dynamical system, and conditioned on background information (demographics, lifestyle, medical history). It forecasts each person's future health and survival from one baseline visit, and learns a network of directed interactions between variables.

## Published fit

| Model | Data | Result |
| --- | --- | --- |
| Network model (Taneja 2016; Farrell 2016) | Human population patterns | Reproduces Gompertz mortality and the average rise of the frailty index with age, with no programmed ageing assumed; Farrell 2016 studies frailty limits and information measures (title) |
| DJIN (Farrell 2022) | ELSA (English Longitudinal Study of Ageing), continuous health variables | Predicts individual multi-variable health trajectories and survival from baseline; flexible low-dimensional latent-space models predicted worse, so ageing health needs many dimensions; can generate realistic synthetic people and impute missing data |

## Known failure modes

- **Human only.** Both were built and fitted on human data. No mouse fit in the sources here (named gap). R-Metric v0 is a mouse measure, so this is a design source, not a calibrated model for UM-HET3.
- **Binary deficits in the first model.** A node is either healthy or damaged, which loses the size of a change. DJIN fixes this with continuous variables.
- **The network is learned, not measured.** In DJIN the interactions are inferred from observational data; they are associations within the model, not proven causal links between organs.
- **Needs long, dense follow-up.** DJIN needs large longitudinal cohorts with the same variables repeated. A mouse equivalent would need the same mice measured repeatedly on many axes, which is the I-2 dataset still being searched for.
- **No mechanism for single interventions.** Neither model names a molecular cause, so it cannot by itself predict the effect of a given drug; it can only propagate a change in one variable through the network.

## What it means for the metric

Ageing is high-dimensional: one summary number loses predictive power (Farrell 2022). This supports a composite metric with many axes (commandment 5), scored per axis and then combined, rather than one marker.

## Sources

- Taneja S et al. Dynamical network model for age-related health deficits and mortality. Phys Rev E 2016. doi:10.1103/PhysRevE.93.022309 (Crossref OK)
- Farrell SG et al. Network model of human aging: frailty limits and information measures. Phys Rev E 2016. doi:10.1103/PhysRevE.94.052409 (Crossref OK)
- Farrell S, Rutenberg A et al. A network perspective of aging and frailty. Innov Aging 2018 (conference abstract). doi:10.1093/geroni/igy023.2643 (Crossref OK)
- Farrell S et al. Interpretable machine learning for high-dimensional trajectories of aging health. PLoS Comput Biol 2022. doi:10.1371/journal.pcbi.1009746 (Crossref OK)
